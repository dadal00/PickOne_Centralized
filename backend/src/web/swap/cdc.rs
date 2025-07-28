use super::{
    models::{Condition, CronItem, CronItemRow, Emoji, Item, ItemType, Location},
    redis::decrement_items,
};
use crate::{
    AppError, AppState, RedisAction,
    microservices::{
        cdc::{get_cdc_date, get_cdc_id, get_cdc_text, get_cdc_u8},
        database::{init::DatabaseQueries, schema::columns::boiler_swap::items},
        meilisearch::delete_item,
        redis::{remove_id, try_get},
    },
};
use anyhow::Result as anyResult;
use chrono::Utc;
use scylla::{client::session::Session, response::PagingState, statement::batch::Batch};
use scylla_cdc::consumer::CDCRow;
use std::{ops::ControlFlow, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::warn;

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

pub fn convert_cdc_item(data: &CDCRow<'_>) -> Item {
    Item {
        item_id: get_cdc_id(data, items::ITEM_ID),
        item_type: ItemType::try_from(get_cdc_u8(data, items::ITEM_TYPE))
            .unwrap_or(ItemType::Other)
            .as_ref()
            .to_string(),
        title: get_cdc_text(data, items::TITLE),
        condition: Condition::try_from(get_cdc_u8(data, items::CONDITION))
            .unwrap_or(Condition::Fair)
            .as_ref()
            .to_string(),
        location: Location::try_from(get_cdc_u8(data, items::LOCATION))
            .unwrap_or(Location::CaryQuadEast)
            .as_ref()
            .to_string(),
        description: get_cdc_text(data, items::DESCRIPTION),
        emoji: Emoji::try_from(get_cdc_u8(data, items::EMOJI))
            .unwrap_or(Emoji::Books)
            .as_ref()
            .to_string(),
        expiration_date: get_cdc_date(data, items::EXPIRATION_DATE),
    }
}

pub async fn handle_item_deletion(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    meili_index: &str,
    redis_deletion_name: &str,
    scylla_id_name: &str,
    website_path: &str,
) -> anyResult<()> {
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
