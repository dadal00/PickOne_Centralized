use super::database::core::insert_item;
use crate::{
    AppError, AppState,
    api::models::{ItemPayload, RedisAction, RedisMetricAction, WebsitePath},
    config::try_load,
};
use once_cell::sync::Lazy;
use redis::{
    AsyncTypedCommands, Client, Script,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{sync::Arc, time::Duration};

static FAILED_ATTEMPTS_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local attempts = redis.call("INCR", KEYS[1])
        if attempts <= tonumber(ARGV[2]) then
            if tonumber(ARGV[1]) > 0 then
                redis.call("EXPIRE", KEYS[1], tonumber(ARGV[1]))
            end
        else
            redis.call("DECR", KEYS[1])
        end
    "#,
    )
});

static DECR_ITEMS_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local attempts = redis.call("DECR", KEYS[1])
        if attempts <= 0 then
            redis.call("DEL", KEYS[1])
        end
    "#,
    )
});

static DECR_METRIC_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local attempts = redis.call("DECR", KEYS[1])
        if attempts < 0 then
            redis.call("SET", KEYS[1], 0)
        end
    "#,
    )
});

static INSERT_SESSION_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        redis.call("SETEX", KEYS[1], tonumber(ARGV[3]), ARGV[2])
        local length = redis.call("LPUSH", KEYS[2], ARGV[1])
        redis.call("EXPIRE", KEYS[2], tonumber(ARGV[3]))
        if length > tonumber(ARGV[4]) then
            local removed_id = redis.call("RPOP", KEYS[2])
            local removed_key = ARGV[5] .. removed_id
            redis.call("DEL", removed_key)
        end
    "#,
    )
});

pub async fn init_redis() -> Result<ConnectionManager, AppError> {
    let redis_url = try_load::<String>("RUST_REDIS_URL", "redis://redis:6379").unwrap();

    let client = Client::open(redis_url)?;

    let config = ConnectionManagerConfig::new()
        .set_number_of_retries(1)
        .set_connection_timeout(Duration::from_millis(100));

    let connection_manager = client.get_connection_manager_with_config(config).await?;
    Ok(connection_manager)
}

pub async fn delete_all_sessions(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    let mut pipe = redis::pipe();

    for session_id in state
        .redis_connection_manager
        .clone()
        .lrange(
            format!("{}:{}:{}", website_path, key_secondary, email),
            0,
            -1,
        )
        .await?
    {
        pipe.del(format!("{}:{}:{}", website_path, key, session_id))
            .ignore();
    }

    pipe.del(format!("{}:{}:{}", website_path, key_secondary, email))
        .ignore();

    pipe.query_async::<()>(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn insert_session(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    session_id: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    let _: () = INSERT_SESSION_SCRIPT
        .key(format!("{}:{}:{}", website_path, key, session_id))
        .key(format!("{}:{}:{}", website_path, key_secondary, email))
        .arg(session_id)
        .arg(email)
        .arg(state.config.session.session_duration_seconds)
        .arg(state.config.session.max_sessions)
        .arg(format!("{}:{}:", website_path, key))
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
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

pub async fn is_temporarily_locked(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    id: &str,
    ttl: i64,
) -> Result<bool, AppError> {
    let result: Option<String> = redis::cmd("SET")
        .arg(format!("{}:{}:{}", website_path, key, id))
        .arg("1")
        .arg("NX")
        .arg("EX")
        .arg(ttl)
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(result.is_none())
}

pub async fn try_get(state: Arc<AppState>, key: &str) -> Result<Option<String>, AppError> {
    Ok(state.redis_connection_manager.clone().get(key).await?)
}

pub async fn increment_lock_key(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    email: &str,
    locked_duration_seconds: &u16,
    max_attempts: &u8,
) -> Result<(), AppError> {
    let _count: () = FAILED_ATTEMPTS_SCRIPT
        .key(format!("{}:{}:{}", website_path, key, email))
        .arg(locked_duration_seconds)
        .arg(max_attempts)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn decrement_items(
    redis_connection_manager: ConnectionManager,
    website_path: &str,
    key: &str,
    email: &str,
) -> Result<(), AppError> {
    let _count: () = DECR_ITEMS_SCRIPT
        .key(format!("{}:{}:{}", website_path, key, email))
        .invoke_async(&mut redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn handle_item_insertion(
    state: Arc<AppState>,
    item: ItemPayload,
    email: &str,
    website_path: &str,
) -> Result<(), AppError> {
    insert_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path,
            RedisAction::DeletedItem.as_ref(),
            &insert_item(state.clone(), item).await?.to_string()
        ),
        email,
        1_209_600,
    )
    .await?;

    increment_lock_key(
        state.clone(),
        website_path,
        RedisAction::LockedItems.as_ref(),
        email,
        &0,
        &state.config.website_specific.max_items,
    )
    .await?;

    Ok(())
}

pub async fn is_redis_locked(
    state: Arc<AppState>,
    website_path: &str,
    key_prefix: &str,
    key_id: &str,
    threshold: &u8,
) -> Result<bool, AppError> {
    if let Some(attempts) = try_get(
        state.clone(),
        &format!("{}:{}:{}", website_path, key_prefix, key_id),
    )
    .await?
    {
        if attempts.parse::<u8>()? >= *threshold {
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn clear_all_keys(
    state: Arc<AppState>,
    website_path: &str,
    keys: &[&str],
    email: &str,
) -> Result<(), AppError> {
    let mut pipe = redis::pipe();

    for key in keys {
        pipe.del(format!("{}:{}:{}", website_path, key, email))
            .ignore();
    }

    pipe.query_async::<()>(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn is_temporarily_locked_ms(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    id: &str,
    ttl_ms: i64,
) -> Result<bool, AppError> {
    let result: Option<String> = redis::cmd("SET")
        .arg(format!("{}:{}:{}", website_path, key, id))
        .arg("1")
        .arg("NX")
        .arg("PX")
        .arg(ttl_ms)
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(result.is_none())
}

pub async fn incr_metric(state: Arc<AppState>, key: &str) -> Result<(), AppError> {
    state.redis_connection_manager.clone().incr(key, 1).await?;

    Ok(())
}

pub async fn incr_visitors(
    state: Arc<AppState>,
    website_path: WebsitePath,
) -> Result<(), AppError> {
    incr_metric(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
            RedisAction::Metric.as_ref(),
            RedisMetricAction::Visitors.as_ref()
        ),
    )
    .await?;

    Ok(())
}

pub async fn decr_metric(state: Arc<AppState>, key: &str) -> Result<(), AppError> {
    let _: () = DECR_METRIC_SCRIPT
        .key(key)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn set_metric(
    mut redis_connection_manager: ConnectionManager,
    key: &str,
    val: &usize,
) -> Result<(), AppError> {
    redis_connection_manager.set(key, val).await?;

    Ok(())
}
