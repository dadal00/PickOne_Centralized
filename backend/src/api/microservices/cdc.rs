use super::{
    database::init::DatabaseQueries,
    meilisearch::{add_items, delete_item},
    redis::{remove_id, try_get},
};
use crate::{
    AppError, AppState,
    api::{
        models::{
            Condition, CronItem, CronItemRow, Emoji, Item, ItemType, Location, RedisAction,
            RedisMetricAction,
        },
        utilities::convert_i8_to_u8,
        web::swap::decrement_items,
    },
    items,
    metrics::{decr_metric, incr_metric},
    tables,
};
use anyhow::Error as anyhowError;
use async_trait::async_trait;
use chrono::{Duration as chronoDuration, NaiveDate, Utc};
use futures_util::future::RemoteHandle;
use once_cell::sync::Lazy;
use scylla::{client::session::Session, response::PagingState, statement::batch::Batch};
use scylla_cdc::{
    checkpoints::TableBackedCheckpointSaver,
    consumer::{CDCRow, Consumer, ConsumerFactory, OperationType},
    log_reader::{CDCLogReader, CDCLogReaderBuilder},
};
use std::{ops::ControlFlow, sync::Arc, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::warn;
use uuid::Uuid;

static BASE_DATE: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

pub struct MeiliConsumer {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub redis_deletion_name: String,
    pub scylla_id_name: String,
    pub website_path: String,
}

impl MeiliConsumer {
    pub async fn new(
        state: Arc<AppState>,
        meili_index: String,
        redis_deletion_name: String,
        scylla_id_name: String,
        website_path: String,
    ) -> Self {
        Self {
            state,
            meili_index,
            redis_deletion_name,
            scylla_id_name,
            website_path,
        }
    }
}

#[async_trait]
impl Consumer for MeiliConsumer {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        match data.operation {
            OperationType::RowInsert => {
                incr_metric(
                    self.state.clone(),
                    &format!(
                        "{}:{}:{}",
                        &self.website_path,
                        RedisAction::Metric.as_ref(),
                        RedisMetricAction::Items.as_ref()
                    ),
                )
                .await?;

                add_items(
                    self.state.meili_client.clone(),
                    &self.meili_index,
                    &[convert_cdc_item(data)],
                    &self.scylla_id_name,
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
                        &self.website_path,
                        RedisAction::Metric.as_ref(),
                        RedisMetricAction::Visitors.as_ref()
                    ),
                )
                .await?;

                handle_item_deletion(
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

pub struct MeiliConsumerFactory {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub redis_deletion_name: String,
    pub scylla_id_name: String,
    pub website_path: String,
}

#[async_trait]
impl ConsumerFactory for MeiliConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(
            MeiliConsumer::new(
                self.state.clone(),
                self.meili_index.clone(),
                self.redis_deletion_name.clone(),
                self.scylla_id_name.clone(),
                self.website_path.clone(),
            )
            .await,
        )
    }
}

async fn handle_item_deletion(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    meili_index: &str,
    redis_deletion_name: &str,
    scylla_id_name: &str,
    website_path: &str,
) -> anyhow::Result<()> {
    let id = get_cdc_id(data, scylla_id_name);

    delete_item(state.meili_client.clone(), meili_index, id).await?;

    decrement_items(
        state.redis_connection_manager.clone(),
        website_path,
        RedisAction::LockedItems.as_ref(),
        &try_get(
            state.clone(),
            &format!(
                "{}:{}:{}",
                website_path,
                redis_deletion_name,
                &id.to_string()
            ),
        )
        .await?
        .expect("item insertion misconfigured"),
    )
    .await?;

    remove_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path,
            redis_deletion_name,
            &id.to_string()
        ),
    )
    .await?;

    Ok(())
}

pub async fn start_cdc(
    state: Arc<AppState>,
    scylla_keyspace: &str,
    scylla_table: &str,
    scylla_id_name: &str,
    redis_deletion_name: &str,
    website_path: &str,
) -> Result<(CDCLogReader, RemoteHandle<Result<(), anyhowError>>), AppError> {
    let items_checkpoint_saver = Arc::new(
        TableBackedCheckpointSaver::new_with_default_ttl(
            state.database_session.clone(),
            scylla_keyspace,
            tables::boiler_swap::CDC,
        )
        .await
        .unwrap(),
    );

    let (cdc_reader, cdc_future) = CDCLogReaderBuilder::new()
        .session(state.database_session.clone())
        .keyspace(scylla_keyspace)
        .table_name(scylla_table)
        .should_save_progress(true)
        .should_load_progress(true)
        .window_size(Duration::from_secs(60))
        .safety_interval(Duration::from_secs(30))
        .sleep_interval(Duration::from_secs(10))
        .pause_between_saves(Duration::from_secs(10))
        .consumer_factory(Arc::new(MeiliConsumerFactory {
            state: state.clone(),
            meili_index: scylla_table.to_string(),
            redis_deletion_name: redis_deletion_name.to_string(),
            scylla_id_name: scylla_id_name.to_string(),
            website_path: website_path.to_string(),
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

pub fn convert_cdc_item(data: CDCRow<'_>) -> Item {
    Item {
        item_id: get_cdc_id(&data, items::ITEM_ID),
        item_type: ItemType::try_from(get_cdc_u8(&data, items::ITEM_TYPE))
            .unwrap_or(ItemType::Other)
            .as_ref()
            .to_string(),
        title: get_cdc_text(&data, items::TITLE),
        condition: Condition::try_from(get_cdc_u8(&data, items::CONDITION))
            .unwrap_or(Condition::Fair)
            .as_ref()
            .to_string(),
        location: Location::try_from(get_cdc_u8(&data, items::LOCATION))
            .unwrap_or(Location::CaryQuadEast)
            .as_ref()
            .to_string(),
        description: get_cdc_text(&data, items::DESCRIPTION),
        emoji: Emoji::try_from(get_cdc_u8(&data, items::EMOJI))
            .unwrap_or(Emoji::Books)
            .as_ref()
            .to_string(),
        expiration_date: get_cdc_date(&data, items::EXPIRATION_DATE),
    }
}

pub async fn spawn_ttl_task(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
) -> Result<(), AppError> {
    let scheduler = JobScheduler::new().await?;

    let database_session = database_session.clone();
    let database_queries = database_queries.clone();

    scheduler
        .add(Job::new_async("1 0 0 * * *", move |_uuid, _lock| {
            let database_session = database_session.clone();
            let database_queries = database_queries.clone();
            Box::pin(async move {
                if expire_ttl(database_session, &database_queries)
                    .await
                    .is_err()
                {
                    warn!("Expiring ttl failed!");
                }
            })
        })?)
        .await?;

    tokio::spawn(async move {
        scheduler.start().await.expect("Failed to start scheduler");
    });

    Ok(())
}

pub fn convert_cron_items(row_vec: &[CronItemRow]) -> Vec<CronItem> {
    row_vec
        .iter()
        .map(|(id, expiration_date)| CronItem {
            item_id: *id,
            expiration_date: *expiration_date,
        })
        .collect()
}

pub async fn expire_ttl(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
) -> Result<(), AppError> {
    let mut paging_state = PagingState::start();

    let mut batch: Batch = Default::default();
    let mut batch_values = Vec::new();

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(
                &database_queries.boiler_swap.get_cron_items,
                &[],
                paging_state,
            )
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<CronItemRow> = row_result
            .rows::<CronItemRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        let items: Vec<CronItem> = convert_cron_items(&row_vec);

        for item in items {
            if item.expiration_date < Utc::now().date_naive() {
                batch.append_statement(database_queries.boiler_swap.delete_item.clone());

                batch_values.push((item.item_id,));
            }
        }

        database_session.batch(&batch, &batch_values).await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break;
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }

    Ok(())
}
