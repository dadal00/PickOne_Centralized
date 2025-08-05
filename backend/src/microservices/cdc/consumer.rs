use super::consumer_utils::{choose_addition, choose_deletion, choose_update};
use crate::{AppState, WebsitePath};
use anyhow::Result as anyResult;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer, ConsumerFactory, OperationType};
use std::sync::Arc;

pub struct MeiliConsumerFactory {
    pub state: Arc<AppState>,
    pub website_path: WebsitePath,
}

#[async_trait]
impl ConsumerFactory for MeiliConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(MeiliConsumer::new(self.state.clone(), self.website_path.clone()).await)
    }
}

pub struct MeiliConsumer {
    pub state: Arc<AppState>,
    pub website_path: WebsitePath,
}

impl MeiliConsumer {
    pub async fn new(state: Arc<AppState>, website_path: WebsitePath) -> Self {
        Self {
            state,
            website_path,
        }
    }
}

#[async_trait]
impl Consumer for MeiliConsumer {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyResult<()> {
        match data.operation {
            OperationType::RowInsert => {
                choose_addition(&data, self.state.meili_client.clone(), &self.website_path).await?;
            }
            OperationType::RowDelete
            | OperationType::PartitionDelete
            | OperationType::RowRangeDelInclLeft
            | OperationType::RowRangeDelExclLeft
            | OperationType::RowRangeDelInclRight
            | OperationType::RowRangeDelExclRight => {
                choose_deletion(&data, self.state.clone(), &self.website_path).await?;
            }
            OperationType::RowUpdate => {
                choose_update(&data, self.state.clone(), &self.website_path).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
