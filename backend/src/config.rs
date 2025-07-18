use crate::AppError;
use std::{env, fmt::Display, fs::read_to_string, str::FromStr};
use tracing::{info, warn};

#[derive(Clone)]
pub struct Server {
    pub rust_port: u16,
    pub svelte_url: String,
}

#[derive(Clone)]
pub struct Email {
    pub from_email: String,
    pub from_email_server: String,
    pub from_email_password: String,
}

#[derive(Clone)]
pub struct Authentication {
    pub auth_max_attempts: u8,
    pub auth_lock_duration_seconds: u16,
    pub verify_max_attempts: u8,
    pub verify_lock_duration_seconds: u16,
    pub max_codes: u8,
    pub max_codes_duration_seconds: u16,
}

#[derive(Clone)]
pub struct Session {
    pub temporary_session_duration_seconds: u16,
    pub max_sessions: u8,
    pub session_duration_seconds: u16,
}

#[derive(Clone)]
pub struct WebsiteSpecific {
    pub max_items: u8,
    pub home_limit_ms: u8,
}

#[derive(Clone)]
pub struct Bot {
    pub num_pictures: u8,
    pub pictures_ttl: u32,
    pub max_bytes: u32,
    pub photo_url: String,
}

#[derive(Clone)]
pub struct Config {
    pub server: Server,
    pub email: Email,
    pub authentication: Authentication,
    pub session: Session,
    pub website_specific: WebsiteSpecific,
    pub bot: Bot,
}

impl Server {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            rust_port: try_load("RUST_PORT", "8080")?,
            svelte_url: try_load("SVELTE_URL", "http://localhost:5173")?,
        })
    }
}

impl Email {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            from_email: read_secret("RUST_FROM_EMAIL").unwrap_or_else(|e| {
                panic!("Failed to load RUST_FROM_EMAIL: {}", e);
            }),
            from_email_server: read_secret("RUST_FROM_EMAIL_SERVER").unwrap_or_else(|e| {
                panic!("Failed to load RUST_FROM_EMAIL_SERVER: {}", e);
            }),
            from_email_password: read_secret("RUST_FROM_EMAIL_PASSWORD").unwrap_or_else(|e| {
                panic!("Failed to load RUST_FROM_EMAIL_PASSWORD: {}", e);
            }),
        })
    }
}

impl Authentication {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            auth_max_attempts: try_load("RUST_AUTH_MAX_ATTEMPTS", "15")?,
            auth_lock_duration_seconds: try_load("RUST_AUTH_LOCK_DURATION_SECS", "1800")?,
            verify_max_attempts: try_load("RUST_VERIFY_MAX_ATTEMPTS", "3")?,
            verify_lock_duration_seconds: try_load("RUST_VERIFY_LOCK_DURATION_SECS", "600")?,
            max_codes: try_load("RUST_MAX_CODES", "5")?,
            max_codes_duration_seconds: try_load("RUST_MAX_CODES_DURATION_SECS", "1800")?,
        })
    }
}

impl Session {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            temporary_session_duration_seconds: try_load(
                "PUBLIC_TEMP_SESSION_DURATION_SECS",
                "600",
            )?,
            max_sessions: try_load("RUST_MAX_SESSIONS", "2")?,
            session_duration_seconds: try_load("RUST_SESSION_DURATION_SECS", "3600")?,
        })
    }
}

impl WebsiteSpecific {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            home_limit_ms: try_load("RUST_HOME_LIMIT_MS", "50")?,
            max_items: try_load("RUST_MAX_ITEMS", "15")?,
        })
    }
}

impl Bot {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            photo_url: try_load("RUST_BOT_PHOTO_URL", "https://boiler/photos")?,
            max_bytes: try_load("RUST_BOT_MAX_BYTES", "5_242_880")?,
            pictures_ttl: try_load("RUST_BOT_PICTURES_TTL", "86400")?,
            num_pictures: try_load("RUST_BOT_NUM_PICTURES", "4")?,
        })
    }
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            server: Server::load()?,
            email: Email::load()?,
            authentication: Authentication::load()?,
            session: Session::load()?,
            website_specific: WebsiteSpecific::load()?,
            bot: Bot::load()?,
        })
    }
}

fn var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|e| {
        warn!("Environment variable {} not found, using default", key);
        AppError::Environment(e)
    })
}

pub fn read_secret(secret_name: &str) -> Result<String, AppError> {
    let path = format!("/run/secrets/{}", secret_name);
    read_to_string(&path)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            warn!("Failed to read {} from file: {}", secret_name, e);
            AppError::IO(e)
        })
}

pub fn try_load<T: FromStr>(key: &str, default: &str) -> Result<T, AppError>
where
    T::Err: Display,
{
    var(key)
        .inspect_err(|_| info!("{key} not set, using default: {default}"))
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .map_err(|e| AppError::Config(format!("Invalid {} value: {}", key, e)))
}
