use super::{
    models::{DummyClaims, RedisAction, VerifiedTokenResult, WebsitePath},
    sessions::get_cookie,
    utilities::format_verified_result,
};
use crate::{AppError, AppState};
use argon2::{
    Algorithm::Argon2id, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version::V0x13, password_hash::SaltString,
};
use axum::http::header::HeaderMap;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use regex::Regex;
use rustrict::CensorStr;
use std::{env, fs::read_to_string, sync::Arc};
use tracing::warn;

pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());
pub static VALIDATION: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::HS256));
pub static SWAP_DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(
        read_to_string("/run/secrets/SWAP_API_TOKEN")
            .map(|s| s.trim().to_string())
            .map_err(|e| {
                warn!("Failed to read SWAP_API_TOKEN from file: {}", e);
                AppError::IO(e)
            })
            .unwrap()
            .as_bytes(),
    )
});
pub static HOME_DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(
        read_to_string("/run/secrets/HOME_API_TOKEN")
            .map(|s| s.trim().to_string())
            .map_err(|e| {
                warn!("Failed to read HOME_API_TOKEN from file: {}", e);
                AppError::IO(e)
            })
            .unwrap()
            .as_bytes(),
    )
});
pub static MAX_CHARS: Lazy<usize> = Lazy::new(|| {
    env::var("PUBLIC_MAX_CHARS")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(100)
});
pub static CODE_LENGTH: Lazy<usize> = Lazy::new(|| {
    env::var("PUBLIC_CODE_LENGTH")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(6)
});
pub static MIN_PASSWORD_LENGTH: Lazy<usize> = Lazy::new(|| {
    env::var("PUBLIC_MIN_PASSWORD_LENGTH")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(10)
});

pub async fn verify_token(
    state: Arc<AppState>,
    headers: HeaderMap,
) -> Result<Option<VerifiedTokenResult>, AppError> {
    if let Some(id) = get_cookie(&headers, RedisAction::Forgot.as_ref()) {
        return format_verified_result(
            state.clone(),
            WebsitePath::BoilerSwap,
            RedisAction::Forgot,
            id,
        )
        .await;
    }

    if let Some(id) = get_cookie(&headers, RedisAction::Auth.as_ref()) {
        return format_verified_result(
            state.clone(),
            WebsitePath::BoilerSwap,
            RedisAction::Auth,
            id,
        )
        .await;
    }

    if let Some(id) = get_cookie(&headers, RedisAction::Update.as_ref()) {
        return format_verified_result(
            state.clone(),
            WebsitePath::BoilerSwap,
            RedisAction::Update,
            id,
        )
        .await;
    }

    if let Some(id) = get_cookie(&headers, RedisAction::Session.as_ref()) {
        return format_verified_result(
            state.clone(),
            WebsitePath::BoilerSwap,
            RedisAction::Session,
            id,
        )
        .await;
    }

    Ok(None)
}

pub fn verify_api_token(headers: HeaderMap, website_path: WebsitePath) -> bool {
    let jwt = get_cookie(&headers, "api_token");

    if jwt.is_none() {
        return false;
    }

    let decoding_key = match website_path {
        WebsitePath::BoilerSwap => &SWAP_DECODING_KEY,
        WebsitePath::Photos => panic!("verify_api_token should not be called for Photos"),
        WebsitePath::Home => &HOME_DECODING_KEY,
    };

    decode::<DummyClaims>(&jwt.expect("is_none failed"), decoding_key, &VALIDATION).is_ok()
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        warn!("Failed to parse password hash: {}", e);
        AppError::Config(e.to_string())
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
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

pub fn validate_account(email: &str, password: &str) -> Result<(), &'static str> {
    validate_email(email)?;

    validate_password(password)?;

    Ok(())
}

pub fn validate_length(payload: &str) -> bool {
    payload.len() < *MAX_CHARS
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if !validate_length(password) && password.len() >= *MIN_PASSWORD_LENGTH {
        return Err("Too many chars");
    }

    if password.is_empty() {
        return Err("Password cannot be empty");
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), &'static str> {
    if !validate_length(email) {
        return Err("Too many chars");
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err("Email must be a Purdue address");
    }

    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(65536, 3, 1, None).map_err(|e| {
        warn!("Failed to hash password: {}", e);
        AppError::Config(e.to_string())
    })?;

    let argon2 = Argon2::new(Argon2id, V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            warn!("Failed to hash password: {}", e);
            AppError::Config(e.to_string())
        })?
        .to_string();

    Ok(password_hash)
}
