use super::{
    cookies::{clear_cookies, remove_cookie},
    locks::{
        check_auth_locks, check_forgot_locks, freeze_account, prepare_resend_and_check_locks,
        unfreeze_account,
    },
    models::{Account, METRICS_ROUTE, PHOTOS_PREFIX, RedisAction, Token},
    sessions::{
        create_forgot_redis_account, create_session, create_temporary_session,
        try_create_redis_account, try_get_redis_account,
    },
    utilities::{get_hashed_ip, get_key, get_website_path},
    verify::{
        check_account, check_email, check_resend, check_token, check_token_content,
        is_request_authorized,
    },
};
use crate::{AppError, AppState};
use axum::{
    Extension, Json,
    extract::{ConnectInfo, Request, State},
    http::{StatusCode, header::HeaderMap},
    middleware::Next,
    response::IntoResponse,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

pub async fn api_token_check(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let path = request.uri().path();

    if path == METRICS_ROUTE || path.starts_with(PHOTOS_PREFIX) {
        return Ok(next.run(request).await);
    }

    is_request_authorized(state.clone(), &headers, &mut request).await?;

    Ok(next.run(request).await)
}

pub async fn forgot_handler(
    Extension(label): Extension<String>,
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    check_email(&payload.token)?;

    let website_path = get_website_path(&label);
    let hashed_ip = get_hashed_ip(&headers, address.ip());

    check_forgot_locks(
        state.clone(),
        &hashed_ip,
        website_path.as_ref(),
        &payload.token,
    )
    .await?;

    let redis_account = create_forgot_redis_account(payload.token);

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            &RedisAction::Forgot,
            &Some(get_key(RedisAction::LockedForgot, &hashed_ip)),
            &Some(get_key(RedisAction::LockedCode, &hashed_ip)),
            &website_path,
        )
        .await?,
    )
        .into_response())
}

pub async fn delete_handler(
    Extension(label): Extension<String>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    remove_cookie(state.clone(), &headers, RedisAction::Session).await?;

    Ok((StatusCode::OK, clear_cookies(&label)).into_response())
}

pub async fn verify_handler(
    Extension(label): Extension<String>,
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    let website_path = get_website_path(&label);

    let verified_result = check_token(
        state.clone(),
        headers.clone(),
        &[RedisAction::Forgot, RedisAction::Update, RedisAction::Auth],
        &website_path,
    )
    .await?;
    check_token_content(&verified_result.redis_action, &payload.token)?;

    let redis_account = try_get_redis_account(
        state.clone(),
        &verified_result,
        &payload.token,
        &get_hashed_ip(&headers, address.ip()),
        &website_path,
    )
    .await?;

    match verified_result.redis_action {
        RedisAction::Forgot => {
            freeze_account(state.clone(), &redis_account.email, &website_path).await?;

            return Ok((
                StatusCode::OK,
                create_temporary_session(
                    state.clone(),
                    &verified_result.serialized_account,
                    &redis_account,
                    &RedisAction::Update,
                    &None,
                    &None,
                    &website_path,
                )
                .await?,
            )
                .into_response());
        }
        RedisAction::Update => {
            unfreeze_account(
                state.clone(),
                &redis_account.email,
                &payload.token,
                &website_path,
            )
            .await?;
        }
        _ => {}
    }

    Ok((
        StatusCode::OK,
        create_session(state.clone(), &redis_account, &website_path).await?,
    )
        .into_response())
}

pub async fn authenticate_handler(
    Extension(label): Extension<String>,
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Account>,
) -> Result<impl IntoResponse, AppError> {
    let website_path = get_website_path(&label);
    let hashed_ip = get_hashed_ip(&headers, address.ip());
    info!("1");
    check_auth_locks(state.clone(), &hashed_ip, website_path.as_ref(), &payload).await?;
    info!("2");
    check_account(&payload)?;
    info!("3");

    let redis_account =
        try_create_redis_account(state.clone(), &hashed_ip, &website_path, &payload).await?;
    info!("4");

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            &RedisAction::Auth,
            &None,
            &Some(get_key(RedisAction::LockedCode, &hashed_ip)),
            &website_path,
        )
        .await?,
    )
        .into_response())
}

pub async fn resend_handler(
    Extension(label): Extension<String>,
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let website_path = get_website_path(&label);

    let verified_result = check_token(
        state.clone(),
        headers.clone(),
        &[RedisAction::Auth, RedisAction::Forgot],
        &website_path,
    )
    .await?;

    let hashed_ip = get_hashed_ip(&headers, address.ip());

    check_resend(&verified_result)?;

    let redis_account = prepare_resend_and_check_locks(
        state.clone(),
        &hashed_ip,
        website_path.as_ref(),
        &verified_result,
    )
    .await?;

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            &verified_result.redis_action,
            &None,
            &Some(get_key(RedisAction::LockedCode, &hashed_ip)),
            &website_path,
        )
        .await?,
    )
        .into_response())
}
