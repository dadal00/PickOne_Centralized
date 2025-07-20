use super::verify::{check_token, validate_length};
use crate::{
    AppError, AppState, RedisAction, WebsitePath,
    api::{
        microservices::redis::{handle_item_insertion, is_redis_locked},
        models::ItemPayload,
    },
};
use axum::{
    Json,
    extract::State,
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
use rustrict::CensorStr;
use std::sync::Arc;

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

pub fn check_item(payload: &ItemPayload) -> Result<(), AppError> {
    validate_item(&payload.title, &payload.description)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok(())
}

pub fn validate_item(title: &str, description: &str) -> Result<(), &'static str> {
    validate_item_attribute(title)?;

    validate_item_attribute(description)?;

    Ok(())
}

pub fn validate_item_attribute(payload: &str) -> Result<(), &'static str> {
    if !validate_length(payload) {
        return Err("Too many chars");
    }

    if payload.is_inappropriate() {
        return Err("Inappropriate");
    }

    Ok(())
}
