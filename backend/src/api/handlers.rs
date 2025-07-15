use super::{
    lock::{freeze_account, unfreeze_account},
    models::{Account, Action, ItemPayload, RedisAccount, RedisAction, Token, WebsitePath},
    redis::{
        clear_all_keys, create_redis_account, get_redis_account, handle_item_insertion,
        increment_lock_key, is_redis_locked, remove_id,
    },
    sessions::{create_session, create_temporary_session, generate_cookie, get_cookie},
    twofactor::{CODE_REGEX, generate_code},
    utilities::{get_hashed_ip, get_key},
    verify::{
        CODE_LENGTH, validate_account, validate_email, validate_item, validate_password,
        verify_api_token, verify_token,
    },
};
use crate::{AppError, state::AppState};
use axum::{
    Json,
    extract::{ConnectInfo, Request, State},
    http::{
        StatusCode,
        header::{HeaderMap, ORIGIN},
    },
    middleware::Next,
    response::IntoResponse,
};
use redis::AsyncTypedCommands;
use std::{net::SocketAddr, sync::Arc};

pub async fn api_token_check(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    if request
        .uri()
        .path()
        .starts_with(&format!("/{}/", WebsitePath::Photos.as_ref()))
    {
        return Ok(next.run(request).await);
    }

    let origin = headers.get(ORIGIN);

    if origin.is_none() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if origin.expect("is_none failed").as_bytes() != state.config.svelte_url.as_bytes() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if verify_api_token(headers) {
        return Ok(next.run(request).await);
    }

    Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response())
}

pub async fn forgot_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    if validate_email(&payload.token).is_err() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let forgot_key = get_key(RedisAction::LockedForgot, &hashed_ip);
    let failed_verify_key = get_key(RedisAction::LockedVerify, &hashed_ip);
    let code_key = get_key(RedisAction::LockedCode, &hashed_ip);

    if is_redis_locked(
        state.clone(),
        WebsitePath::BoilerSwap.as_ref(),
        &failed_verify_key,
        &payload.token,
        &state.config.verify_max_attempts,
    )
    .await?
        || is_redis_locked(
            state.clone(),
            WebsitePath::BoilerSwap.as_ref(),
            &code_key,
            &payload.token,
            &state.config.max_codes,
        )
        .await?
    {
        return Ok((StatusCode::UNAUTHORIZED, "Try again in 30 minutes").into_response());
    }

    let redis_account = RedisAccount {
        email: payload.token,
        action: Action::Forgot,
        code: generate_code().clone(),
        issued_timestamp: None,
        password_hash: None,
    };

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            RedisAction::Forgot,
            &Some(forgot_key),
            &Some(code_key),
            WebsitePath::BoilerSwap,
        )
        .await?,
    )
        .into_response())
}

pub async fn delete_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let id = get_cookie(&headers, RedisAction::Session.as_ref());

    if id.is_some() {
        state
            .redis_connection_manager
            .clone()
            .del(format!(
                "{}:{}",
                RedisAction::Session.as_ref(),
                id.expect("is_none failed")
            ))
            .await?;
    }

    Ok((
        StatusCode::OK,
        generate_cookie("", "", 0, WebsitePath::BoilerSwap),
    )
        .into_response())
}

pub async fn verify_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    let verified_result = match verify_token(state.clone(), headers.clone()).await? {
        Some(verified_result) => {
            if verified_result.redis_action == RedisAction::Session {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
            verified_result
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if verified_result.redis_action == RedisAction::Update
        && validate_password(&payload.token).is_err()
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if (verified_result.redis_action == RedisAction::Auth
        || verified_result.redis_action == RedisAction::Forgot)
        && (payload.token.len() != *CODE_LENGTH || !CODE_REGEX.is_match(&payload.token))
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let forgot_key = get_key(RedisAction::LockedForgot, &hashed_ip);
    let failed_verify_key = get_key(RedisAction::LockedVerify, &hashed_ip);
    let failed_auth_key = get_key(RedisAction::LockedAuth, &hashed_ip);
    let code_key = get_key(RedisAction::LockedCode, &hashed_ip);

    let redis_account = match get_redis_account(
        state.clone(),
        &verified_result,
        &payload.token,
        RedisAction::LockedTemporary,
        &failed_verify_key,
        WebsitePath::BoilerSwap.as_ref(),
    )
    .await?
    {
        Some(account) => {
            clear_all_keys(
                state.clone(),
                WebsitePath::BoilerSwap.as_ref(),
                &[&code_key, &forgot_key, &failed_auth_key, &failed_verify_key],
                &account.email,
            )
            .await?;
            account
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if verified_result.redis_action == RedisAction::Forgot {
        freeze_account(state.clone(), &redis_account.email, WebsitePath::BoilerSwap).await?;

        return Ok((
            StatusCode::OK,
            create_temporary_session(
                state.clone(),
                &verified_result.serialized_account,
                &redis_account,
                RedisAction::Update,
                &None,
                &None,
                WebsitePath::BoilerSwap,
            )
            .await?,
        )
            .into_response());
    }

    if verified_result.redis_action == RedisAction::Update {
        unfreeze_account(state.clone(), &redis_account.email, &payload.token).await?;
    }

    Ok((
        StatusCode::OK,
        create_session(
            state.clone(),
            &redis_account,
            RedisAction::Session,
            RedisAction::SessionStore,
            WebsitePath::BoilerSwap,
        )
        .await?,
    )
        .into_response())
}

pub async fn authenticate_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Account>,
) -> Result<impl IntoResponse, AppError> {
    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let failed_auth_key = get_key(RedisAction::LockedAuth, &hashed_ip);
    let code_key = get_key(RedisAction::LockedCode, &hashed_ip);

    if is_redis_locked(
        state.clone(),
        WebsitePath::BoilerSwap.as_ref(),
        &failed_auth_key,
        &payload.email,
        &state.config.auth_max_attempts,
    )
    .await?
        || is_redis_locked(
            state.clone(),
            WebsitePath::BoilerSwap.as_ref(),
            &code_key,
            &payload.email,
            &state.config.max_codes,
        )
        .await?
    {
        return Ok((StatusCode::UNAUTHORIZED, "Try again in 30 minutes").into_response());
    }

    if payload.action == Action::Forgot {
        increment_lock_key(
            state.clone(),
            WebsitePath::BoilerSwap.as_ref(),
            &failed_auth_key,
            &payload.email,
            &state.config.auth_lock_duration_seconds,
            &state.config.auth_max_attempts,
        )
        .await?;
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if let Err(e) = validate_account(&payload.email, &payload.password) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    let redis_account = match create_redis_account(
        state.clone(),
        payload.action,
        &payload.email,
        &payload.password,
        &failed_auth_key,
        WebsitePath::BoilerSwap.as_ref(),
    )
    .await?
    {
        Some(account) => {
            remove_id(
                state.clone(),
                &format!(
                    "{}:{}:{}",
                    WebsitePath::BoilerSwap.as_ref(),
                    &failed_auth_key,
                    &payload.email
                ),
            )
            .await?;
            account
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            RedisAction::Auth,
            &None,
            &Some(code_key),
            WebsitePath::BoilerSwap,
        )
        .await?,
    )
        .into_response())
}

pub async fn post_item_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    let email = match verify_token(state.clone(), headers.clone()).await? {
        Some(verified_result) => {
            if verified_result.redis_action != RedisAction::Session {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
            verified_result.serialized_account
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if let Err(e) = validate_item(&payload.title, &payload.description) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    if is_redis_locked(
        state.clone(),
        WebsitePath::BoilerSwap.as_ref(),
        RedisAction::LockedItems.as_ref(),
        &email.clone().expect("session creation faulty"),
        &state.config.max_items,
    )
    .await?
    {
        return Ok((StatusCode::UNAUTHORIZED, "Posted too many items").into_response());
    }

    handle_item_insertion(
        state.clone(),
        payload,
        &email.expect("session creation faulty"),
        WebsitePath::BoilerSwap.as_ref(),
    )
    .await?;

    Ok((StatusCode::OK).into_response())
}

pub async fn resend_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let verified_result = match verify_token(state.clone(), headers.clone()).await? {
        Some(verified_result) => {
            if verified_result.redis_action != RedisAction::Auth
                && verified_result.redis_action != RedisAction::Forgot
            {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
            verified_result
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if verified_result.serialized_account.is_none() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    remove_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            WebsitePath::BoilerSwap.as_ref(),
            verified_result.redis_action.as_ref(),
            &verified_result.id
        ),
    )
    .await?;

    let redis_account: RedisAccount =
        serde_json::from_str(&verified_result.serialized_account.expect("is_none failed"))?;
    let hashed_ip = get_hashed_ip(&headers, address.ip());
    let code_key = get_key(RedisAction::LockedCode, &hashed_ip);

    if is_redis_locked(
        state.clone(),
        WebsitePath::BoilerSwap.as_ref(),
        &code_key,
        &redis_account.email,
        &state.config.max_codes,
    )
    .await?
    {
        return Ok((StatusCode::UNAUTHORIZED, "Try again in 30 minutes").into_response());
    }

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            verified_result.redis_action,
            &None,
            &Some(code_key),
            WebsitePath::BoilerSwap,
        )
        .await?,
    )
        .into_response())
}
