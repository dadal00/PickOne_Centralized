use super::consumer_utils::{choose_addition, choose_deletion, choose_update};
use crate::{AppState, WebsitePath};
use anyhow::Result as anyResult;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer, ConsumerFactory, OperationType};
use std::sync::Arc;

pub struct MeiliConsumerFactory {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub scylla_id_name: String,
    pub website_path: WebsitePath,
    pub redis_deletion_name: Option<String>,
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
                self.redis_deletion_name.clone(),
            )
            .await,
        )
    }
}

pub struct MeiliConsumer {
    pub state: Arc<AppState>,
    pub meili_index: String,
    pub scylla_id_name: String,
    pub website_path: WebsitePath,
    pub redis_deletion_name: Option<String>,
}

impl MeiliConsumer {
    pub async fn new(
        state: Arc<AppState>,
        meili_index: String,
        scylla_id_name: String,
        website_path: WebsitePath,
        redis_deletion_name: Option<String>,
    ) -> Self {
        Self {
            state,
            meili_index,
            scylla_id_name,
            website_path,
            redis_deletion_name,
        }
    }
}

#[async_trait]
impl Consumer for MeiliConsumer {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyResult<()> {
        match data.operation {
            OperationType::RowInsert => {
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
                choose_deletion(
                    &data,
                    self.state.clone(),
                    &self.meili_index,
                    &self.redis_deletion_name.as_deref(),
                    &self.scylla_id_name,
                    &self.website_path,
                )
                .await?;
            }
            OperationType::RowUpdate => {
                choose_update(
                    &data,
                    self.state.meili_client.clone(),
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
