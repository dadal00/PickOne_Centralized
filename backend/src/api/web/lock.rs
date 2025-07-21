use super::{twofactor::generate_code, verify::hash_password};
use crate::{
    AppError, AppState,
    api::{
        microservices::{
            database::core::{check_lock, get_user, unlock_account, update_lock},
            redis::{
                delete_all_sessions, increment_lock_key, is_redis_locked, is_temporarily_locked_ms,
                remove_id, try_get,
            },
        },
        models::{
            Account, Action, LockCheck, RedisAccount, RedisAction, VerifiedTokenResult, WebsitePath,
        },
        utilities::get_key,
    },
};
use chrono::{Duration as chronoDuration, Utc};
use redis::AsyncTypedCommands;
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub async fn check_db_lock(state: Arc<AppState>, email: &str) -> Result<bool, AppError> {
    let locked = check_lock(state.clone(), email).await?;

    if locked.is_some() && locked.expect("is_some failed") {
        return Ok(true);
    }
    Ok(false)
}

pub async fn freeze_account(
    state: Arc<AppState>,
    email: &str,
    website_path: WebsitePath,
) -> Result<(), AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(());
    }

    state
        .redis_connection_manager
        .clone()
        .set_ex(
            format!("{}:{}", RedisAction::LockedTime.as_ref(), &email),
            (Utc::now() + chronoDuration::milliseconds(500)).timestamp_millis(),
            900,
        )
        .await?;

    update_lock(state.clone(), email, true).await?;

    delete_all_sessions(
        state.clone(),
        website_path.as_ref(),
        RedisAction::Session.as_ref(),
        RedisAction::SessionStore.as_ref(),
        email,
    )
    .await?;

    Ok(())
}

pub async fn unfreeze_account(
    state: Arc<AppState>,
    email: &str,
    password: &str,
) -> Result<(), AppError> {
    let password_owned = password.to_owned();

    unlock_account(
        state.clone(),
        email,
        &spawn_blocking(move || hash_password(&password_owned)).await?,
    )
    .await?;

    Ok(())
}

pub async fn check_locks(
    state: Arc<AppState>,
    email: &str,
    issued_timestamp: i64,
    website_path: &str,
) -> Result<bool, AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(true);
    }

    let locked_timestamp = try_get(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path,
            RedisAction::LockedTime.as_ref(),
            email
        ),
    )
    .await?;

    if locked_timestamp.is_some()
        && issued_timestamp
            < locked_timestamp
                .expect("is_some failed")
                .parse::<i64>()
                .unwrap_or(i64::MAX)
    {
        return Ok(true);
    }

    Ok(false)
}

pub async fn are_all_locked(
    state: Arc<AppState>,
    website_path: &str,
    token: &str,
    locks: &[LockCheck<'_>],
) -> Result<bool, AppError> {
    for lock in locks {
        if is_redis_locked(state.clone(), website_path, lock.key, token, lock.check).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn check_forgot_lock(
    state: Arc<AppState>,
    email: &str,
    forgot_key: &Option<String>,
    website_path: &str,
) -> bool {
    if let Some(key) = forgot_key {
        match get_user(state.clone(), email).await {
            Ok(Some(_)) => (),
            _ => return true,
        }

        if let Ok(is_locked) = is_redis_locked(
            state.clone(),
            website_path,
            key,
            email,
            &state.config.authentication.verify_max_attempts,
        )
        .await
        {
            if is_locked {
                return true;
            }
        }
    }

    false
}

pub async fn check_forgot_locks(
    state: Arc<AppState>,
    hashed_ip: &str,
    website_path: &str,
    token: &str,
) -> Result<(), AppError> {
    if are_all_locked(
        state.clone(),
        website_path,
        token,
        &[
            LockCheck {
                key: &get_key(RedisAction::LockedVerify, hashed_ip),
                check: &state.config.authentication.verify_max_attempts,
            },
            LockCheck {
                key: &get_key(RedisAction::LockedCode, hashed_ip),
                check: &state.config.authentication.max_codes,
            },
        ],
    )
    .await?
    {
        return Err(AppError::Unauthorized(
            "Try again in 30 minutes".to_string(),
        ));
    }

    Ok(())
}

pub async fn prepare_resend_and_check_locks(
    state: Arc<AppState>,
    hashed_ip: &str,
    website_path: &str,
    verified_result: &VerifiedTokenResult,
) -> Result<RedisAccount, AppError> {
    remove_id(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path,
            verified_result.redis_action.as_ref(),
            &verified_result.id
        ),
    )
    .await?;

    let old_redis_account: RedisAccount = serde_json::from_str(
        &verified_result
            .serialized_account
            .clone()
            .expect("is_none failed"),
    )?;

    if are_all_locked(
        state.clone(),
        website_path,
        &old_redis_account.email,
        &[LockCheck {
            key: &get_key(RedisAction::LockedCode, hashed_ip),
            check: &state.config.authentication.max_codes,
        }],
    )
    .await?
    {
        return Err(AppError::Unauthorized(
            "Try again in 30 minutes".to_string(),
        ));
    }

    Ok(RedisAccount {
        code: generate_code(),
        ..old_redis_account
    })
}

pub async fn check_auth_locks(
    state: Arc<AppState>,
    hashed_ip: &str,
    website_path: &str,
    payload: &Account,
) -> Result<(), AppError> {
    if are_all_locked(
        state.clone(),
        website_path,
        &payload.email,
        &[
            LockCheck {
                key: &get_key(RedisAction::LockedAuth, hashed_ip),
                check: &state.config.authentication.auth_max_attempts,
            },
            LockCheck {
                key: &get_key(RedisAction::LockedCode, hashed_ip),
                check: &state.config.authentication.max_codes,
            },
        ],
    )
    .await?
    {
        return Err(AppError::Unauthorized(
            "Try again in 30 minutes".to_string(),
        ));
    }

    if payload.action != Action::Forgot {
        return Ok(());
    }

    increment_lock_key(
        state.clone(),
        website_path,
        &get_key(RedisAction::LockedAuth, hashed_ip),
        &payload.email,
        &state.config.authentication.auth_lock_duration_seconds,
        &state.config.authentication.auth_max_attempts,
    )
    .await?;

    Err(AppError::Unauthorized("Invalid Credentials".to_string()))
}

pub async fn is_home_locked(state: Arc<AppState>, hashed_ip: &str) -> Result<(), AppError> {
    if is_temporarily_locked_ms(
        state.clone(),
        WebsitePath::Home.as_ref(),
        RedisAction::LockedTemporary.as_ref(),
        hashed_ip,
        state.config.website_specific.home_limit_ms.into(),
    )
    .await?
    {
        return Err(AppError::Unauthorized(
            "Too many requests from your ip".to_string(),
        ));
    }
    Ok(())
}
