use super::{
    models::{Account, Action, RedisAccount, RedisAction, VerifiedTokenResult, WebsitePath},
    sessions::delete_all_sessions,
    swap::database::{check_lock, get_user, unlock_account, update_lock},
    twofactor::generate_code,
    utilities::get_key,
    verify::hash_password,
};
use crate::{
    AppError,
    AppError::HttpResponseBack,
    AppState,
    error::HttpErrorResponse::Unauthorized,
    microservices::redis::{insert_id, remove_id, try_get},
};
use chrono::{Duration as chronoDuration, Utc};
use once_cell::sync::Lazy;
use redis::Script;
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub struct LockCheck<'a> {
    pub key: &'a str,
    pub check: &'a u8,
}

static FAILED_ATTEMPTS_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local attempts = redis.call("INCR", KEYS[1])
        if attempts <= tonumber(ARGV[2]) then
            if tonumber(ARGV[1]) > 0 then
                redis.call("EXPIRE", KEYS[1], tonumber(ARGV[1]))
            end
        else
            redis.call("DECR", KEYS[1])
        end
    "#,
    )
});

pub async fn check_db_lock(
    state: Arc<AppState>,
    email: &str,
    website_path: &WebsitePath,
) -> Result<bool, AppError> {
    Ok(check_lock(state.clone(), email, website_path)
        .await?
        .unwrap_or(false))
}

pub async fn freeze_account(
    state: Arc<AppState>,
    email: &str,
    website_path: &WebsitePath,
) -> Result<(), AppError> {
    if check_db_lock(state.clone(), email, website_path).await? {
        return Ok(());
    }

    insert_id(
        state.clone(),
        &format!("{}:{}", RedisAction::LockedTime.as_ref(), &email),
        &(Utc::now() + chronoDuration::milliseconds(500))
            .timestamp_millis()
            .to_string(),
        900,
    )
    .await?;

    update_lock(state.clone(), email, true, website_path).await?;

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
    website_path: &WebsitePath,
) -> Result<(), AppError> {
    unlock_account(
        state.clone(),
        email,
        &spawn_blocking({
            let password_owned = password.to_owned();
            move || hash_password(&password_owned)
        })
        .await?,
        website_path,
    )
    .await?;

    Ok(())
}

pub async fn check_locks(
    state: Arc<AppState>,
    email: &str,
    issued_timestamp: i64,
    website_path: &WebsitePath,
) -> Result<bool, AppError> {
    if check_db_lock(state.clone(), email, website_path).await? {
        return Ok(true);
    }

    let locked_timestamp = try_get(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
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
    website_path: &WebsitePath,
) -> bool {
    if let Some(key) = forgot_key {
        match get_user(state.clone(), email, website_path).await {
            Ok(Some(_)) => (),
            _ => return true,
        }

        if let Ok(is_locked) = is_redis_locked(
            state.clone(),
            website_path.as_ref(),
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
        return Err(HttpResponseBack(Unauthorized(
            "Try again in 30 minutes".to_string(),
        )));
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
        return Err(HttpResponseBack(Unauthorized(
            "Try again in 30 minutes".to_string(),
        )));
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
        return Err(HttpResponseBack(Unauthorized(
            "Try again in 30 minutes".to_string(),
        )));
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

    Err(HttpResponseBack(Unauthorized(
        "Invalid Credentials".to_string(),
    )))
}

pub async fn increment_lock_key(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    email: &str,
    locked_duration_seconds: &u16,
    max_attempts: &u8,
) -> Result<(), AppError> {
    let _count: () = FAILED_ATTEMPTS_SCRIPT
        .key(format!("{website_path}:{key}:{email}"))
        .arg(locked_duration_seconds)
        .arg(max_attempts)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn is_redis_locked(
    state: Arc<AppState>,
    website_path: &str,
    key_prefix: &str,
    key_id: &str,
    threshold: &u8,
) -> Result<bool, AppError> {
    match try_get(state, &format!("{website_path}:{key_prefix}:{key_id}")).await? {
        Some(attempts) => Ok(attempts.parse::<u8>()? >= *threshold),
        None => Ok(false),
    }
}

pub async fn is_temporarily_locked(
    state: Arc<AppState>,
    website_path: &str,
    key: &str,
    id: &str,
    ttl: i64,
) -> Result<bool, AppError> {
    let result: Option<String> = redis::cmd("SET")
        .arg(format!("{website_path}:{key}:{id}"))
        .arg("1")
        .arg("NX")
        .arg("EX")
        .arg(ttl)
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(result.is_none())
}
