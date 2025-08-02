use super::{
    cookies::generate_cookie,
    locks::{check_locks, increment_lock_key, is_redis_locked, is_temporarily_locked},
    models::{Account, Action, RedisAccount, RedisAction, VerifiedTokenResult, WebsitePath},
    swap::database::{get_user, insert_user},
    twofactor::{generate_code, spawn_code_task},
    utilities::{clear_all_keys, get_key},
    verify::{hash_password, verify_password},
};
use crate::{
    AppError,
    AppError::HttpResponseBack,
    AppState,
    error::{HttpErrorResponse, HttpErrorResponse::Unauthorized},
    microservices::redis::{insert_id, remove_id},
};
use axum::http::header::HeaderMap;
use chrono::Utc;
use once_cell::sync::Lazy;
use redis::{AsyncTypedCommands, Script};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use uuid::Uuid;

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

pub async fn create_temporary_session(
    state: Arc<AppState>,
    result: &Option<String>,
    redis_account: &RedisAccount,
    redis_action: &RedisAction,
    forgot_key: &Option<String>,
    code_key: &Option<String>,
    website_path: &WebsitePath,
) -> Result<HeaderMap, AppError> {
    send_code(
        state.clone(),
        redis_action,
        redis_account,
        forgot_key,
        code_key,
        website_path,
    )
    .await?;

    let serialized = match result {
        Some(result) => result,
        None => &serde_json::to_string(&redis_account)?,
    };

    let id = Uuid::new_v4().to_string();

    insert_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
            redis_action.as_ref(),
            &id
        ),
        serialized,
        state
            .config
            .session
            .temporary_session_duration_seconds
            .into(),
    )
    .await?;

    Ok(generate_cookie(
        redis_action.as_ref(),
        &id,
        state
            .config
            .session
            .temporary_session_duration_seconds
            .into(),
        website_path,
    ))
}

async fn send_code(
    state: Arc<AppState>,
    redis_action: &RedisAction,
    redis_account: &RedisAccount,
    forgot_key: &Option<String>,
    code_key: &Option<String>,
    website_path: &WebsitePath,
) -> Result<(), AppError> {
    if *redis_action == RedisAction::Update {
        return Ok(());
    }

    spawn_code_task(
        state.clone(),
        redis_account.email.clone(),
        redis_account.code.clone(),
        forgot_key.clone(),
        website_path.clone(),
    );

    increment_lock_key(
        state.clone(),
        website_path.as_ref(),
        code_key.as_ref().unwrap(),
        &redis_account.email,
        &state.config.authentication.max_codes_duration_seconds,
        &state.config.authentication.max_codes,
    )
    .await?;

    Ok(())
}

pub async fn create_session(
    state: Arc<AppState>,
    redis_account: &RedisAccount,
    website_path: &WebsitePath,
) -> Result<HeaderMap, AppError> {
    if redis_account.action == Action::Signup {
        insert_user(state.clone(), redis_account, website_path).await?;
    }

    let session_id = Uuid::new_v4().to_string();

    insert_session(
        state.clone(),
        website_path.as_ref(),
        RedisAction::Session.as_ref(),
        &session_id,
        RedisAction::SessionStore.as_ref(),
        &redis_account.email,
    )
    .await?;

    Ok(generate_cookie(
        RedisAction::Session.as_ref(),
        &session_id,
        state.config.session.session_duration_seconds.into(),
        website_path,
    ))
}

pub async fn try_create_redis_account(
    state: Arc<AppState>,
    hashed_ip: &str,
    website_path: &WebsitePath,
    payload: &Account,
) -> Result<RedisAccount, AppError> {
    let lock_key = get_key(RedisAction::LockedAuth, hashed_ip);

    let account = create_redis_account(
        state.clone(),
        payload.action.clone(),
        &payload.email,
        &payload.password,
        &lock_key,
        website_path,
    )
    .await?
    .ok_or(HttpErrorResponse::Unauthorized(
        "Unable to verify".to_string(),
    ))?;

    remove_id(
        state.clone(),
        &format!("{}:{}:{}", website_path.as_ref(), &lock_key, &payload.email),
    )
    .await?;

    Ok(account)
}

pub async fn get_redis_account(
    state: Arc<AppState>,
    verified_result: &VerifiedTokenResult,
    code: &str,
    redis_action_secondary: RedisAction,
    failed_verify_key: &str,
    website_path: &WebsitePath,
) -> Result<Option<RedisAccount>, AppError> {
    let serialized = match &verified_result.serialized_account {
        Some(s) => s,
        None => return Ok(None),
    };

    if is_temporarily_locked(
        state.clone(),
        website_path.as_ref(),
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
        website_path.as_ref(),
        failed_verify_key,
        &deserialized.email,
        &state.config.authentication.verify_max_attempts,
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

    if !locked && verified_result.redis_action != RedisAction::Update && code != deserialized.code {
        increment_lock_key(
            state.clone(),
            website_path.as_ref(),
            failed_verify_key,
            &deserialized.email,
            &state.config.authentication.verify_lock_duration_seconds,
            &state.config.authentication.verify_max_attempts,
        )
        .await?;

        return Ok(None);
    }

    remove_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
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

pub async fn create_redis_account(
    state: Arc<AppState>,
    action: Action,
    email: &str,
    password: &str,
    failed_auth_key: &str,
    website_path: &WebsitePath,
) -> Result<Option<RedisAccount>, AppError> {
    match get_user(state.clone(), email, website_path).await? {
        None => {
            if action == Action::Login {
                return Ok(None);
            }

            let password_hash = spawn_blocking({
                let password_owned = password.to_owned();
                move || hash_password(&password_owned)
            })
            .await?;

            Ok(Some(create_auth_redis_account(
                email.to_string(),
                action.clone(),
                Some(password_hash),
            )))
        }
        Some((_, locked)) if action == Action::Signup || locked => Ok(None),
        Some((hash, _)) => {
            if action == Action::Login
                && !spawn_blocking({
                    let plaintext = password.to_owned();
                    let hash = hash.to_owned();
                    move || verify_password(&plaintext, &hash)
                })
                .await?
            {
                increment_lock_key(
                    state.clone(),
                    website_path.as_ref(),
                    failed_auth_key,
                    email,
                    &state.config.authentication.auth_lock_duration_seconds,
                    &state.config.authentication.auth_max_attempts,
                )
                .await?;

                return Ok(None);
            }

            Ok(Some(create_auth_redis_account(
                email.to_string(),
                action.clone(),
                None,
            )))
        }
    }
}

pub async fn try_get_redis_account(
    state: Arc<AppState>,
    verified_result: &VerifiedTokenResult,
    token: &str,
    hashed_ip: &str,
    website_path: &WebsitePath,
) -> Result<RedisAccount, AppError> {
    match get_redis_account(
        state.clone(),
        verified_result,
        token,
        RedisAction::LockedTemporary,
        &get_key(RedisAction::LockedVerify, hashed_ip),
        website_path,
    )
    .await?
    {
        Some(account) => {
            clear_all_keys(
                state.clone(),
                website_path.as_ref(),
                &[
                    &get_key(RedisAction::LockedCode, hashed_ip),
                    &get_key(RedisAction::LockedForgot, hashed_ip),
                    &get_key(RedisAction::LockedAuth, hashed_ip),
                    &get_key(RedisAction::LockedVerify, hashed_ip),
                ],
                &account.email,
            )
            .await?;

            Ok(account)
        }
        None => Err(HttpResponseBack(Unauthorized(
            "Unable to verify".to_string(),
        ))),
    }
}

pub fn create_forgot_redis_account(email: String) -> RedisAccount {
    RedisAccount {
        email,
        action: Action::Forgot,
        code: generate_code().clone(),
        issued_timestamp: None,
        password_hash: None,
    }
}

pub fn create_auth_redis_account(
    email: String,
    action: Action,
    password_hash: Option<String>,
) -> RedisAccount {
    RedisAccount {
        email,
        action,
        code: generate_code().clone(),
        issued_timestamp: Some(Utc::now().timestamp_millis()),
        password_hash,
    }
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
        .lrange(format!("{website_path}:{key_secondary}:{email}"), 0, -1)
        .await?
    {
        pipe.del(format!("{website_path}:{key}:{session_id}"))
            .ignore();
    }

    pipe.del(format!("{website_path}:{key_secondary}:{email}"))
        .ignore();

    pipe.query_async::<()>(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

async fn insert_session(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    session_id: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    let _: () = INSERT_SESSION_SCRIPT
        .key(format!("{website_path}:{key}:{session_id}"))
        .key(format!("{website_path}:{key_secondary}:{email}"))
        .arg(session_id)
        .arg(email)
        .arg(state.config.session.session_duration_seconds)
        .arg(state.config.session.max_sessions)
        .arg(format!("{website_path}:{key}:"))
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}
