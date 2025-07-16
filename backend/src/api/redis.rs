use super::{
    database::{get_user, insert_item},
    lock::check_locks,
    models::{Action, ItemPayload, RedisAccount, RedisAction, VerifiedTokenResult},
    twofactor::generate_code,
    verify::{hash_password, verify_password},
};
use crate::{AppError, AppState};
use chrono::Utc;
use once_cell::sync::Lazy;
use redis::{
    AsyncTypedCommands, Client, Script,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{env, sync::Arc, time::Duration};
use tokio::task::spawn_blocking;
use tracing::warn;

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
    let redis_url = env::var("RUST_REDIS_URL").unwrap_or_else(|_| {
        warn!("Environment variable RUST_REDIS_URL not found, using default");
        "redis://redis:6379".to_string()
    });

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
        .arg(state.config.session_duration_seconds)
        .arg(state.config.max_sessions)
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

pub async fn get_redis_account(
    state: Arc<AppState>,
    verified_result: &VerifiedTokenResult,
    code: &str,
    redis_action_secondary: RedisAction,
    failed_verify_key: &str,
    website_path: &str,
) -> Result<Option<RedisAccount>, AppError> {
    match &verified_result.serialized_account {
        Some(serialized) => {
            if is_temporarily_locked(
                state.clone(),
                website_path,
                redis_action_secondary.as_ref(),
                &verified_result.id,
                1,
            )
            .await?
            {
                return Ok(None);
            }

            let deserialized: RedisAccount = serde_json::from_str(serialized)?;

            if is_redis_locked(
                state.clone(),
                website_path,
                failed_verify_key,
                &deserialized.email,
                &state.config.verify_max_attempts,
            )
            .await?
            {
                return Ok(None);
            }

            let locked = match verified_result.redis_action {
                RedisAction::Auth => {
                    check_locks(
                        state.clone(),
                        &deserialized.email,
                        deserialized.issued_timestamp.expect("auth account"),
                        website_path,
                    )
                    .await?
                }
                _ => false,
            };

            if !locked
                && verified_result.redis_action != RedisAction::Update
                && code != deserialized.code
            {
                increment_lock_key(
                    state.clone(),
                    website_path,
                    failed_verify_key,
                    &deserialized.email,
                    &state.config.verify_lock_duration_seconds,
                    &state.config.verify_max_attempts,
                )
                .await?;
                return Ok(None);
            }

            remove_id(
                state.clone(),
                &format!(
                    "{}:{}:{}",
                    website_path,
                    verified_result.redis_action.as_ref(),
                    &verified_result.id
                ),
            )
            .await?;

            if locked {
                return Ok(None);
            }

            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}

pub async fn create_redis_account(
    state: Arc<AppState>,
    action: Action,
    email: &str,
    password: &str,
    failed_auth_key: &str,
    website_path: &str,
) -> Result<Option<RedisAccount>, AppError> {
    match get_user(state.clone(), email).await? {
        None => {
            if action == Action::Login {
                return Ok(None);
            }

            let password_owned = password.to_owned();

            let password_hash = spawn_blocking(move || hash_password(&password_owned)).await??;

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: Some(password_hash),
            }))
        }
        Some((hash, locked)) => {
            let plaintext = password.to_owned();

            let hash = hash.to_owned();

            if action == Action::Signup || locked {
                return Ok(None);
            }

            if action == Action::Login
                && !spawn_blocking(move || verify_password(&plaintext, &hash)).await??
            {
                increment_lock_key(
                    state.clone(),
                    website_path,
                    failed_auth_key,
                    email,
                    &state.config.auth_lock_duration_seconds,
                    &state.config.auth_max_attempts,
                )
                .await?;
                return Ok(None);
            }

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: None,
            }))
        }
    }
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
        &state.config.max_items,
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
