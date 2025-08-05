use crate::{AppError, AppState};
use once_cell::sync::Lazy;
use redis::{AsyncTypedCommands, Script, cmd};
use std::sync::Arc;

static INSERT_USER_PHOTO_BYTES_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local length = redis.call("LPUSH", KEYS[1], ARGV[1])
        redis.call("EXPIRE", KEYS[1], tonumber(ARGV[3]))
        if length > tonumber(ARGV[2]) then
            redis.call("RPOP", KEYS[1])
            length = length - 1
        end
        return length
    "#,
    )
});

static INSERT_FORMATTED_PHOTO_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local old_qr_id = redis.call("GET", KEYS[2])
        if old_qr_id then
            redis.call("DEL", ARGV[5] .. ":" .. old_qr_id)
        end
        redis.call("SET", KEYS[1], ARGV[3], "EX", ARGV[4])
        redis.call("SET", KEYS[2], ARGV[2], "EX", ARGV[4])
        "#,
    )
});

pub async fn insert_user_photo_bytes(
    state: Arc<AppState>,
    key_prefix: &str,
    user_id: &str,
    photo_bytes: &[u8],
) -> Result<u8, AppError> {
    let length: u8 = INSERT_USER_PHOTO_BYTES_SCRIPT
        .key(format!("{key_prefix}:{user_id}"))
        .arg(photo_bytes)
        .arg(state.config.bot.num_pictures)
        .arg(state.config.bot.pictures_ttl)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(length)
}

pub async fn insert_formatted_photo(
    state: Arc<AppState>,
    key_prefix: &str,
    photo_id: &str,
    photo_bytes: &[u8],
    key_secondary_prefix: &str,
    user_id: &str,
) -> Result<(), AppError> {
    let _: () = INSERT_FORMATTED_PHOTO_SCRIPT
        .key(format!("{key_prefix}:{photo_id}"))
        .key(format!("{key_prefix}:{key_secondary_prefix}:{user_id}"))
        .arg(user_id)
        .arg(photo_id)
        .arg(photo_bytes)
        .arg(state.config.bot.pictures_ttl.to_string())
        .arg(key_prefix)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn get_num_photos(
    state: Arc<AppState>,
    key_prefix: &str,
    user_id: &str,
) -> Result<u8, AppError> {
    Ok(state
        .redis_connection_manager
        .clone()
        .llen(format!("{key_prefix}:{user_id}"))
        .await?
        .try_into()
        .unwrap())
}

pub async fn get_vector_of_bytes(
    state: Arc<AppState>,
    key_prefix: &str,
    user_id: &str,
) -> Result<Vec<Vec<u8>>, AppError> {
    let vector_of_bytes: Vec<Vec<u8>> = cmd("LRANGE")
        .arg(format!("{key_prefix}:{user_id}"))
        .arg(0)
        .arg(-1)
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(vector_of_bytes)
}

pub async fn get_bytes(
    state: Arc<AppState>,
    key_prefix: &str,
    id: &str,
) -> Result<Option<Vec<u8>>, AppError> {
    let bytes: Option<Vec<u8>> = cmd("GET")
        .arg(format!("{key_prefix}:{id}"))
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(bytes)
}
