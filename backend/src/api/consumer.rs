use super::{
    database::{convert_cdc_item, get_cdc_id},
    meilisearch::{add_items, delete_item},
    models::RedisAction,
    redis::{decrement_items, remove_id, try_get},
};
use crate::state::AppState;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer, ConsumerFactory, OperationType};
use std::sync::Arc;

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
                self.state.metrics.swap_products.inc();

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
                self.state.metrics.swap_products.dec();

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
