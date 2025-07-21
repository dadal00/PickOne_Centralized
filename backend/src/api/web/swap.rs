use super::verify::{check_token, validate_length};
use crate::{
    AppError, AppState, RedisAction, WebsitePath,
    api::{
        microservices::redis::insert_id,
        models::{Condition, Emoji, Item, ItemPayload, ItemRow, ItemType, Location},
        utilities::convert_i8_to_u8,
        web::lock::{increment_lock_key, is_redis_locked},
    },
};
use axum::{
    Json,
    extract::State,
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
use chrono::{Duration as chronoDuration, Utc};
use once_cell::sync::Lazy;
use redis::{Script, aio::ConnectionManager};
use rustrict::CensorStr;
use scylla::response::PagingState;
use std::sync::Arc;
use uuid::Uuid;

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

pub async fn post_item_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    let email = check_token(
        state.clone(),
        headers.clone(),
        &[RedisAction::Session],
        &WebsitePath::BoilerSwap,
    )
    .await?
    .serialized_account;

    check_item(&payload)?;

    try_post_item(
        state.clone(),
        &email.expect("session creation faulty"),
        payload,
    )
    .await?;

    Ok((StatusCode::OK).into_response())
}

async fn try_post_item(
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
        return Err(AppError::Unauthorized("Posted too many items".to_string()));
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

fn check_item(payload: &ItemPayload) -> Result<(), AppError> {
    validate_item(&payload.title, &payload.description)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok(())
}

fn validate_item(title: &str, description: &str) -> Result<(), &'static str> {
    validate_item_attribute(title)?;

    validate_item_attribute(description)?;

    Ok(())
}

fn validate_item_attribute(payload: &str) -> Result<(), &'static str> {
    if !validate_length(payload) {
        return Err("Too many chars");
    }

    if payload.is_inappropriate() {
        return Err("Inappropriate");
    }

    Ok(())
}

async fn handle_item_insertion(
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

pub async fn insert_item(state: Arc<AppState>, item: ItemPayload) -> Result<Uuid, AppError> {
    let fallback_page_state = PagingState::start();
    let id = Uuid::new_v4();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.insert_item,
            (
                &id,
                item.item_type as i8,
                item.title,
                item.condition as i8,
                item.location as i8,
                item.description,
                item.emoji as i8,
                Utc::now().date_naive() + chronoDuration::days(7),
                604800 * 3,
            ),
            fallback_page_state,
        )
        .await?;

    Ok(id)
}

pub fn convert_db_items(row_vec: &Vec<ItemRow>) -> Vec<Item> {
    row_vec
        .iter()
        .map(
            |(
                id,
                item_type_i8,
                title,
                condition_i8,
                location_i8,
                description,
                emoji_i8,
                expiration_date,
            )| Item {
                item_id: *id,
                item_type: ItemType::try_from(convert_i8_to_u8(item_type_i8))
                    .unwrap_or(ItemType::Other)
                    .as_ref()
                    .to_string(),
                title: title.to_string(),
                condition: Condition::try_from(convert_i8_to_u8(condition_i8))
                    .unwrap_or(Condition::Fair)
                    .as_ref()
                    .to_string(),
                location: Location::try_from(convert_i8_to_u8(location_i8))
                    .unwrap_or(Location::CaryQuadEast)
                    .as_ref()
                    .to_string(),
                description: description.to_string(),
                emoji: Emoji::try_from(convert_i8_to_u8(emoji_i8))
                    .unwrap_or(Emoji::Books)
                    .as_ref()
                    .to_string(),
                expiration_date: expiration_date.format("%Y-%m-%d").to_string(),
            },
        )
        .collect()
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
