use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisAccount {
    pub email: String,
    pub action: Action,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_timestamp: Option<i64>,
    pub password_hash: Option<String>,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Clone)]
pub enum Action {
    Login,
    Signup,
    Forgot,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone)]
pub enum RedisAction {
    #[strum(serialize = "auth_id")]
    Auth,

    #[strum(serialize = "forgot_id")]
    Forgot,

    #[strum(serialize = "locked_timestamp")]
    LockedTime,

    #[strum(serialize = "session_id")]
    Session,

    #[strum(serialize = "temporary_lock")]
    LockedTemporary,

    #[strum(serialize = "update_id")]
    Update,

    #[strum(serialize = "sessions")]
    SessionStore,

    #[strum(serialize = "verify_lock")]
    LockedVerify,

    #[strum(serialize = "auth_lock")]
    LockedAuth,

    #[strum(serialize = "forgot_lock")]
    LockedForgot,

    #[strum(serialize = "code_lock")]
    LockedCode,

    #[strum(serialize = "item_lock")]
    LockedItems,

    #[strum(serialize = "item_deleted")]
    DeletedItem,

    #[strum(serialize = "metric")]
    Metric,
}

#[derive(Deserialize)]
pub struct DummyClaims {}

#[derive(EnumString, AsRefStr, PartialEq, Clone)]
pub enum WebsitePath {
    #[strum(serialize = "swap")]
    BoilerSwap,

    #[strum(serialize = "housing")]
    Housing,

    #[strum(serialize = "photos")]
    Photos,

    #[strum(serialize = "home")]
    Home,
}

pub struct VerifiedTokenResult {
    pub serialized_account: Option<String>,
    pub redis_action: RedisAction,
    pub id: String,
}

#[derive(Serialize)]
pub struct VisitorPayload {
    pub website: String,
    pub visitors: u64,
}

#[derive(EnumString, AsRefStr)]
pub enum WebsiteRoute {
    #[strum(serialize = "api")]
    Api,

    #[strum(serialize = "authenticate")]
    Authenticate,

    #[strum(serialize = "verify")]
    Verify,

    #[strum(serialize = "delete")]
    Delete,

    #[strum(serialize = "forgot")]
    Forgot,

    #[strum(serialize = "resend")]
    Resend,
}

pub const METRICS_ROUTE: &str = "/metrics";

pub const PHOTOS_PREFIX: &str = "/photos/";
