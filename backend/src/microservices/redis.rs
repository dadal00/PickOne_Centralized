use crate::{AppError, AppState, config::try_load};
use redis::{
    AsyncTypedCommands, Client,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{sync::Arc, time::Duration};

pub async fn init_redis() -> Result<ConnectionManager, AppError> {
    let redis_url = try_load::<String>("RUST_REDIS_URL", "redis://redis:6379").unwrap();

    let client = Client::open(redis_url)?;

    let config = ConnectionManagerConfig::new()
        .set_number_of_retries(1)
        .set_connection_timeout(Duration::from_millis(100));

    let connection_manager = client.get_connection_manager_with_config(config).await?;

    Ok(connection_manager)
}

pub async fn insert_id(
    state: Arc<AppState>,
    key: &str,
    value: &str,
    ttl: u32,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .set_ex(key, value, ttl.into())
        .await?;

    Ok(())
}

pub async fn remove_id(state: Arc<AppState>, key: &str) -> Result<(), AppError> {
    state.redis_connection_manager.clone().del(key).await?;

    Ok(())
}

pub async fn try_get(state: Arc<AppState>, key: &str) -> Result<Option<String>, AppError> {
    Ok(state.redis_connection_manager.clone().get(key).await?)
}
