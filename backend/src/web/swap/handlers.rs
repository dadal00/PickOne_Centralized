use super::{models::ItemPayload, redis::try_post_item, verify::check_item};
use crate::{AppError, AppState, RedisAction, WebsitePath, web::verify::check_token};
use axum::{
    Json,
    extract::State,
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
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
