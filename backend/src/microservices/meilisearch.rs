use super::database::DatabaseQueries;
use crate::{
    AppError,
    config::{read_secret, try_load},
    web::{housing::init::init_housing, swap::init::init_swap},
};
use anyhow::Result as anyResult;
use meilisearch_sdk::client::*;
use scylla::client::session::Session;
use serde::Serialize;
use std::{
    marker::{Send, Sync},
    sync::Arc,
};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub async fn init_meilisearch(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
) -> Result<
    (
        Arc<Client>,
        JoinHandle<anyResult<()>>,
        JoinHandle<anyResult<()>>,
    ),
    AppError,
> {
    let meili_url = try_load::<String>("MEILI_URL", "http://meilisearch:7700")?;

    let meili_client = Arc::new(Client::new(
        meili_url,
        Some(read_secret("MEILI_ADMIN_KEY")?),
    )?);

    Ok((
        meili_client.clone(),
        init_swap(
            database_session.clone(),
            database_queries,
            meili_client.clone(),
        )
        .await?,
        init_housing(database_session, database_queries, meili_client).await?,
    ))
}

pub async fn add_items<T>(
    meili_client: Arc<Client>,
    index_name: &str,
    items: &[T],
    id_name: &str,
) -> anyResult<()>
where
    T: Serialize + Send + Sync,
{
    meili_client
        .index(index_name)
        .add_documents(items, Some(id_name))
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

pub async fn delete_item(meili_client: Arc<Client>, index_name: &str, key: Uuid) -> anyResult<()> {
    meili_client
        .index(index_name)
        .delete_document(key)
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

pub async fn update_items<T>(
    meili_client: Arc<Client>,
    index_name: &str,
    items: &[T],
    id_name: &str,
) -> anyResult<()>
where
    T: Serialize + Send + Sync,
{
    meili_client
        .index(index_name)
        .add_or_update(items, Some(id_name))
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

pub async fn clear_index(meili_client: Arc<Client>, index_name: &str) -> anyResult<()> {
    meili_client
        .index(index_name)
        .delete_all_documents()
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}
