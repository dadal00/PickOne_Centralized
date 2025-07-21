use crate::{
    AppError,
    api::{
        microservices::{
            cdc::{expire_ttl, spawn_ttl_task},
            database::init::{DatabaseQueries, init_database},
            meilisearch::init_meilisearch,
            redis::init_redis,
        },
        models::{RedisAction, RedisMetricAction, WebsitePath},
    },
    config::Config,
    metrics::set_redis_metric,
};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;
use scylla::client::session::Session;
use std::sync::{Arc, atomic::Ordering::Relaxed};
use tokio::task::JoinHandle;

pub struct AppState {
    pub config: Config,
    pub database_session: Arc<Session>,
    pub database_queries: DatabaseQueries,
    pub redis_connection_manager: ConnectionManager,
    pub meili_client: Arc<Client>,
}

impl AppState {
    pub async fn new() -> Result<(Arc<Self>, JoinHandle<Result<(), AppError>>), AppError> {
        let redis_future = init_redis();

        let (database_session, database_queries) = init_database().await?;
        expire_ttl(database_session.clone(), &database_queries).await?;
        let expire_ttl_future = spawn_ttl_task(database_session.clone(), &database_queries);

        let meili_future = init_meilisearch(database_session.clone(), &database_queries);

        let config = Config::load()?;

        let redis_connection_manager = redis_future.await?;
        expire_ttl_future.await?;
        let (meili_client, meili_reindex_future, item_counter) = meili_future.await?;

        set_redis_metric(
            redis_connection_manager.clone(),
            &format!(
                "{}:{}:{}",
                WebsitePath::BoilerSwap.as_ref(),
                RedisAction::Metric.as_ref(),
                RedisMetricAction::Items.as_ref()
            ),
            &item_counter.load(Relaxed),
        )
        .await?;

        Ok((
            Arc::new(Self {
                config,
                database_session,
                database_queries,
                redis_connection_manager,
                meili_client,
            }),
            meili_reindex_future,
        ))
    }
}
