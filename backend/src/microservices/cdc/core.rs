use super::consumer::MeiliConsumerFactory;
use crate::{AppError, AppState, WebsitePath};
use anyhow::Error as anyhowError;
use chrono::NaiveDate;
use futures_util::future::RemoteHandle;
use once_cell::sync::Lazy;
use scylla_cdc::{
    checkpoints::TableBackedCheckpointSaver,
    log_reader::{CDCLogReader, CDCLogReaderBuilder},
};
use std::{sync::Arc, time::Duration};

pub static BASE_DATE: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

#[derive(Clone)]
pub struct ScyllaCDCParams {
    pub keyspace: String,
    pub table: String,
}

pub async fn start_cdc(
    state: Arc<AppState>,
    scylla_params: ScyllaCDCParams,
    website_path: WebsitePath,
    cdc_table_name: &str,
) -> Result<(CDCLogReader, RemoteHandle<Result<(), anyhowError>>), AppError> {
    let items_checkpoint_saver = Arc::new(
        TableBackedCheckpointSaver::new_with_default_ttl(
            state.database_session.clone(),
            &scylla_params.keyspace,
            cdc_table_name,
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
            website_path,
        }))
        .checkpoint_saver(items_checkpoint_saver)
        .build()
        .await?;

    Ok((cdc_reader, cdc_future))
}
