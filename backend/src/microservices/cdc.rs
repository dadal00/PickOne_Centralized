use super::meilisearch::add_items;
use crate::{
    AppError, AppState, WebsitePath,
    metrics::{decr_metric, incr_metric},
    tables,
    web::swap::cdc::{convert_cdc_item, handle_item_deletion},
};
use anyhow::{Error as anyhowError, Result as anyResult};
use async_trait::async_trait;
use chrono::{Duration as chronoDuration, NaiveDate};
use futures_util::future::RemoteHandle;
use meilisearch_sdk::client::*;
use once_cell::sync::Lazy;
use scylla_cdc::{
    checkpoints::TableBackedCheckpointSaver,
    consumer::{CDCRow, Consumer, ConsumerFactory, OperationType},
    log_reader::{CDCLogReader, CDCLogReaderBuilder},
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

static BASE_DATE: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

#[derive(Clone)]
pub struct ScyllaCDCParams {
    pub keyspace: String,
    pub table: String,
    pub id_name: String,
}

#[derive(Clone)]
pub struct RedisCDCParams {
    pub metric: String,
    pub deletion_name: String,
    pub metric_prefix: String,
}

pub struct MeiliConsumer {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub scylla_id_name: String,
    pub website_path: WebsitePath,
    pub redis_metric: String,
    pub redis_deletion_name: String,
    pub redis_metric_prefix: String,
}

impl MeiliConsumer {
    pub async fn new(
        state: Arc<AppState>,
        meili_index: String,
        scylla_id_name: String,
        website_path: WebsitePath,
        redis_metric: String,
        redis_deletion_name: String,
        redis_metric_prefix: String,
    ) -> Self {
        Self {
            state,
            meili_index,
            scylla_id_name,
            website_path,
            redis_metric,
            redis_deletion_name,
            redis_metric_prefix,
        }
    }
}

#[async_trait]
impl Consumer for MeiliConsumer {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyResult<()> {
        match data.operation {
            OperationType::RowInsert => {
                incr_metric(
                    self.state.clone(),
                    &format!(
                        "{}:{}:{}",
                        self.website_path.as_ref(),
                        &self.redis_metric_prefix,
                        &self.redis_metric,
                    ),
                )
                .await?;

                choose_addition(
                    &data,
                    self.state.meili_client.clone(),
                    &self.meili_index,
                    &self.scylla_id_name,
                    &self.website_path,
                )
                .await?;
            }
            OperationType::RowDelete
            | OperationType::PartitionDelete
            | OperationType::RowRangeDelInclLeft
            | OperationType::RowRangeDelExclLeft
            | OperationType::RowRangeDelInclRight
            | OperationType::RowRangeDelExclRight => {
                decr_metric(
                    self.state.clone(),
                    &format!(
                        "{}:{}:{}",
                        self.website_path.as_ref(),
                        &self.redis_metric_prefix,
                        &self.redis_metric,
                    ),
                )
                .await?;

                choose_deletion(
                    &data,
                    self.state.clone(),
                    &self.meili_index,
                    &self.redis_deletion_name,
                    &self.scylla_id_name,
                    &self.website_path,
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }
}

async fn choose_addition(
    data: &CDCRow<'_>,
    meili_client: Arc<Client>,
    meili_index: &str,
    scylla_id_name: &str,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => {
            add_items(
                meili_client,
                meili_index,
                &[convert_cdc_item(data)],
                scylla_id_name,
            )
            .await
        }
        WebsitePath::Photos => panic!("Photos not implemented"),
        WebsitePath::Home => panic!("Home not implemented"),
        WebsitePath::Housing => panic!("Housing not implemented"),
    }
}

async fn choose_deletion(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    meili_index: &str,
    redis_deletion_name: &str,
    scylla_id_name: &str,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => Ok(handle_item_deletion(
            data,
            state,
            meili_index,
            redis_deletion_name,
            scylla_id_name,
            website_path.as_ref(),
        )
        .await?),
        WebsitePath::Photos => panic!("Photos not implemented"),
        WebsitePath::Home => panic!("Home not implemented"),
        WebsitePath::Housing => panic!("Housing not implemented"),
    }
}

pub struct MeiliConsumerFactory {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub scylla_id_name: String,
    pub website_path: WebsitePath,
    pub redis_metric: String,
    pub redis_deletion_name: String,
    pub redis_metric_prefix: String,
}

#[async_trait]
impl ConsumerFactory for MeiliConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(
            MeiliConsumer::new(
                self.state.clone(),
                self.meili_index.clone(),
                self.scylla_id_name.clone(),
                self.website_path.clone(),
                self.redis_metric.clone(),
                self.redis_deletion_name.clone(),
                self.redis_metric_prefix.clone(),
            )
            .await,
        )
    }
}

pub async fn start_cdc(
    state: Arc<AppState>,
    scylla_params: ScyllaCDCParams,
    website_path: WebsitePath,
    redis_params: RedisCDCParams,
) -> Result<(CDCLogReader, RemoteHandle<Result<(), anyhowError>>), AppError> {
    let items_checkpoint_saver = Arc::new(
        TableBackedCheckpointSaver::new_with_default_ttl(
            state.database_session.clone(),
            &scylla_params.keyspace,
            tables::boiler_swap::CDC,
        )
        .await
        .unwrap(),
    );

    let (cdc_reader, cdc_future) = CDCLogReaderBuilder::new()
        .session(state.database_session.clone())
        .keyspace(&scylla_params.keyspace)
        .table_name(&scylla_params.table)
        .should_save_progress(true)
        .should_load_progress(true)
        .window_size(Duration::from_secs(60))
        .safety_interval(Duration::from_secs(30))
        .sleep_interval(Duration::from_secs(10))
        .pause_between_saves(Duration::from_secs(10))
        .consumer_factory(Arc::new(MeiliConsumerFactory {
            state: state.clone(),
            meili_index: scylla_params.table,
            scylla_id_name: scylla_params.id_name,
            website_path,
            redis_metric: redis_params.metric,
            redis_deletion_name: redis_params.deletion_name,
            redis_metric_prefix: redis_params.metric_prefix,
        }))
        .checkpoint_saver(items_checkpoint_saver)
        .build()
        .await?;

    Ok((cdc_reader, cdc_future))
}

pub fn get_cdc_id(data: &CDCRow<'_>, id_name: &str) -> Uuid {
    data.get_value(id_name)
        .as_ref()
        .and_then(|v| v.as_uuid())
        .expect("Missing item id")
}

pub fn get_cdc_tinyint(data: &CDCRow<'_>, column: &str) -> i8 {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_tinyint())
        .expect("Missing tinyint attribute")
}

pub fn get_cdc_u8(data: &CDCRow<'_>, column: &str) -> u8 {
    convert_i8_to_u8(&get_cdc_tinyint(data, column))
}

pub fn get_cdc_text(data: &CDCRow<'_>, column: &str) -> String {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_text())
        .expect("Missing text attribute")
        .to_string()
}

pub fn get_cdc_date(data: &CDCRow<'_>, column: &str) -> String {
    let days = data
        .get_value(column)
        .as_ref()
        .and_then(|v| v.as_cql_date())
        .expect("Missing date attribute")
        .0 as i64;

    BASE_DATE
        .checked_add_signed(chronoDuration::days(days - 2_147_483_648))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .expect("Missing the date attribute!")
}

pub fn convert_i8_to_u8(payload: &i8) -> u8 {
    payload.checked_abs().unwrap_or(0) as u8
}
