use super::{
    cookies::get_cookie,
    models::{Account, DummyClaims, VerifiedTokenResult},
    twofactor::CODE_REGEX,
    utilities::{format_verified_result, label_request},
};
use crate::{
    AppError, AppState, RedisAction, WebsitePath, WebsiteRoute,
    config::{read_secret, try_load},
    metrics::incr_visitors,
};
use argon2::{
    Algorithm::Argon2id, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version::V0x13, password_hash::SaltString,
};
use axum::{
    extract::Request,
    http::header::{HeaderMap, ORIGIN},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use regex::Regex;
use std::sync::Arc;

pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());

pub static VALIDATION: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::HS256));

pub static SWAP_DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| read_decoding_key("SWAP_API_TOKEN"));

pub static HOUSING_DECODING_KEY: Lazy<DecodingKey> =
    Lazy::new(|| read_decoding_key("HOUSING_API_TOKEN"));

pub static HOME_DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| read_decoding_key("HOME_API_TOKEN"));

pub static MAX_CHARS: Lazy<usize> = Lazy::new(|| try_load("PUBLIC_MAX_CHARS", "100").unwrap());

pub static CODE_LENGTH: Lazy<usize> = Lazy::new(|| try_load("PUBLIC_CODE_LENGTH", "6").unwrap());

pub static MIN_PASSWORD_LENGTH: Lazy<usize> =
    Lazy::new(|| try_load("PUBLIC_MIN_PASSWORD_LENGTH", "10").unwrap());

pub async fn verify_token(
    state: Arc<AppState>,
    headers: HeaderMap,
    website_path: &WebsitePath,
) -> Result<Option<VerifiedTokenResult>, AppError> {
    let actions = [
        RedisAction::Forgot,
        RedisAction::Auth,
        RedisAction::Update,
        RedisAction::Session,
    ];

    for action in &actions {
        if let Some(id) = get_cookie(&headers, action.as_ref()) {
            return format_verified_result(state.clone(), website_path, action.clone(), id).await;
        }
    }

    Ok(None)
}

pub fn verify_api_token(headers: &HeaderMap, website_path: &WebsitePath) -> bool {
    let jwt = match get_cookie(headers, "api_token") {
        Some(token) => token,
        None => return false,
    };

    let decoding_key = match website_path {
        WebsitePath::BoilerSwap => &SWAP_DECODING_KEY,
        WebsitePath::Housing => &HOUSING_DECODING_KEY,
        WebsitePath::Photos => panic!("verify_api_token should not be called for Photos"),
        WebsitePath::Home => &HOME_DECODING_KEY,
    };

    decode::<DummyClaims>(&jwt, decoding_key, &VALIDATION).is_ok()
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash)
        .unwrap_or_else(|e| panic!("Failed to parse password hash: {}", e));

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
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

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(65536, 3, 1, None)
        .unwrap_or_else(|e| panic!("Failed to create Argon2 params: {}", e));

    let argon2 = Argon2::new(Argon2id, V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap_or_else(|e| panic!("Failed to hash password: {}", e))
        .to_string();

    password_hash
}

fn read_decoding_key(secret_name: &str) -> DecodingKey {
    DecodingKey::from_secret(
        read_secret(secret_name)
            .unwrap_or_else(|e| {
                panic!("Failed to load {}: {}", secret_name, e);
            })
            .as_bytes(),
    )
}

pub async fn check_token(
    state: Arc<AppState>,
    headers: HeaderMap,
    allowed: &[RedisAction],
    website_path: &WebsitePath,
) -> Result<VerifiedTokenResult, AppError> {
    match verify_token(state.clone(), headers.clone(), website_path).await? {
        Some(verified_result) if allowed.contains(&verified_result.redis_action) => {
            Ok(verified_result)
        }
        _ => Err(AppError::Unauthorized("Unable to verify".to_string())),
    }
}

pub fn check_token_content(redis_action: &RedisAction, token: &str) -> Result<(), AppError> {
    match redis_action {
        RedisAction::Update => validate_password(token)
            .map_err(|_| AppError::Unauthorized("Unable to verify".to_string())),
        RedisAction::Auth | RedisAction::Forgot
            if token.len() != *CODE_LENGTH || !CODE_REGEX.is_match(token) =>
        {
            Err(AppError::Unauthorized("Unable to verify".to_string()))
        }
        _ => Ok(()),
    }
}

pub async fn is_request_authorized(
    state: Arc<AppState>,
    headers: &HeaderMap,
    request: &mut Request,
) -> Result<(), AppError> {
    let origin = headers
        .get(ORIGIN)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Unable to verify".to_string()))?;

    if origin != state.config.server.svelte_url {
        return Err(AppError::Unauthorized("Unable to verify".to_string()));
    }

    let website_path = check_path(state.clone(), request)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Unable to verify".to_string()))?;

    if !verify_api_token(headers, &website_path) {
        return Err(AppError::Unauthorized("Unable to verify".to_string()));
    }

    Ok(())
}

pub fn check_email(token: &str) -> Result<(), AppError> {
    validate_email(token).map_err(|e| AppError::BadRequest(e.to_string()))
}

pub fn check_account(payload: &Account) -> Result<(), AppError> {
    validate_account(&payload.email, &payload.password)
        .map_err(|e| AppError::BadRequest(e.to_string()))
}

pub fn check_resend(payload: &VerifiedTokenResult) -> Result<(), AppError> {
    payload
        .serialized_account
        .as_ref()
        .ok_or(AppError::Unauthorized("Unable to verify".to_string()))?;

    Ok(())
}

async fn check_path(
    state: Arc<AppState>,
    request: &mut Request,
) -> Result<Option<WebsitePath>, AppError> {
    match request.uri().path() {
        path if path.starts_with(&format!(
            "/{}/{}",
            WebsitePath::BoilerSwap.as_ref(),
            WebsiteRoute::Api.as_ref()
        )) =>
        {
            incr_visitors(state.clone(), WebsitePath::BoilerSwap).await?;

            label_request(request, WebsitePath::BoilerSwap);

            Ok(Some(WebsitePath::BoilerSwap))
        }
        path if path.starts_with(&format!(
            "/{}/{}",
            WebsitePath::Housing.as_ref(),
            WebsiteRoute::Api.as_ref()
        )) =>
        {
            incr_visitors(state.clone(), WebsitePath::Housing).await?;

            Ok(Some(WebsitePath::Housing))
        }
        path if path.starts_with(&format!(
            "/{}/{}",
            WebsitePath::Home.as_ref(),
            WebsiteRoute::Api.as_ref()
        )) =>
        {
            incr_visitors(state.clone(), WebsitePath::Home).await?;

            Ok(Some(WebsitePath::Home))
        }
        _ => Ok(None),
    }
}
