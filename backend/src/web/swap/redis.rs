use super::{
    database::{insert_email, insert_item},
    models::ItemPayload,
};
use crate::{
    AppError,
    AppError::HttpResponseBack,
    AppState, RedisAction, WebsitePath,
    error::HttpErrorResponse::Unauthorized,
    web::locks::{increment_lock_key, is_redis_locked},
};
use once_cell::sync::Lazy;
use redis::{Script, aio::ConnectionManager};
use std::sync::Arc;

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

async fn handle_item_insertion(
    state: Arc<AppState>,
    item: ItemPayload,
    email: &str,
    website_path: &str,
) -> Result<(), AppError> {
    insert_email(
        state.clone(),
        &insert_item(state.clone(), item).await?,
        email,
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

pub async fn decrement_items(
    redis_connection_manager: ConnectionManager,
    website_path: &str,
    key: &str,
    email: &str,
) -> Result<(), AppError> {
    let _count: () = DECR_ITEMS_SCRIPT
        .key(format!("{website_path}:{key}:{email}"))
        .invoke_async(&mut redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn try_post_item(
    state: Arc<AppState>,
    email: &str,
    payload: ItemPayload,
) -> Result<(), AppError> {
    if is_redis_locked(
        state.clone(),
        WebsitePath::BoilerSwap.as_ref(),
        RedisAction::LockedItems.as_ref(),
        email,
        &state.config.website_specific.max_items,
    )
    .await?
    {
        return Err(HttpResponseBack(Unauthorized(
            "Posted too many items".to_string(),
        )));
    }

    handle_item_insertion(
        state.clone(),
        payload,
        email,
        WebsitePath::BoilerSwap.as_ref(),
    )
    .await?;

    Ok(())
}
