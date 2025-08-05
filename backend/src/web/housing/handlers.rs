use super::{
    database::{insert_review, update_thumbs},
    models::{ReviewPayload, ThumbsDeltaMap},
    verify::check_review,
};
use crate::{AppError, AppState};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn post_review_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ReviewPayload>,
) -> Result<impl IntoResponse, AppError> {
    check_review(&payload)?;

    insert_review(state, payload).await?;

    Ok((StatusCode::OK).into_response())
}

pub async fn update_thumbs_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ThumbsDeltaMap>,
) -> Result<impl IntoResponse, AppError> {
    update_thumbs(state, payload).await?;

    Ok((StatusCode::OK).into_response())
}
