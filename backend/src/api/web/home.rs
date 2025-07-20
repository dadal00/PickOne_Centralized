use super::lock::is_home_locked;
use crate::{AppError, AppState, api::utilities::get_hashed_ip, metrics::get_visitors_payload};
use axum::{
    extract::{ConnectInfo, State},
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
use std::{net::SocketAddr, sync::Arc};

pub async fn visitors_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    is_home_locked(state.clone(), &get_hashed_ip(&headers, address.ip())).await?;

    Ok((StatusCode::OK, get_visitors_payload(state.clone()).await?).into_response())
}
