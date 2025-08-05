use anyhow::Error as anyhowError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use image::ImageError;
use lettre::{
    address::AddressError, error::Error as lettreGeneralError,
    transport::smtp::Error as lettreTransportError,
};
use meilisearch_sdk::errors::Error as meiliError;
use prometheus::Error as prometheusError;
use redis::RedisError;
use reqwest::Error as reqwestError;
use scylla::{
    deserialize::DeserializationError,
    errors::{
        ExecutionError, FirstRowError, IntoRowsResultError, NewSessionError, PrepareError,
        RowsError,
    },
};
use serde_json::Error as serdeJsonError;
use std::{env::VarError, io::Error as IOError, num::ParseIntError, string::FromUtf8Error};
use strum::ParseError as strumError;
use teloxide::RequestError as teloxideRequestError;
use thiserror::Error;
use tokio::task::JoinError;
use tokio_cron_scheduler::JobSchedulerError;
use tracing::error;

#[derive(Error, Debug)]
pub enum ScyllaError {
    #[error("ScyllaDB new session error: {0}")]
    Init(#[from] NewSessionError),

    #[error("ScyllaDB execution error: {0}")]
    Execute(#[from] ExecutionError),

    #[error("ScyllaDB prepare error: {0}")]
    Prepare(#[from] PrepareError),

    #[error("ScyllaDB rows result error: {0}")]
    RowsResult(#[from] IntoRowsResultError),

    #[error("ScyllaDB first row error: {0}")]
    FirstRow(#[from] FirstRowError),

    #[error("ScyllaDB row error: {0}")]
    RowsError(#[from] RowsError),

    #[error("ScyllaDB deserialization error: {0}")]
    DeserializationError(#[from] DeserializationError),
}

#[derive(Error, Debug)]
pub enum HttpErrorResponse {
    #[error("Invalid Credentials: {0}")]
    Unauthorized(String),

    #[error("Malformed payload: {0}")]
    BadRequest(String),
}

#[derive(Error, Debug)]
pub enum BotError {
    #[error("ImageProcessing error: {0}")]
    ImageProcessing(#[from] ImageError),

    #[error("TeloxideRequest error: {0}")]
    TeloxideRequest(#[from] teloxideRequestError),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Environment error: {0}")]
    Environment(#[from] VarError),

    #[error("IO error: {0}")]
    IO(#[from] IOError),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Configuration(#[from] ConfigError),

    #[error(transparent)]
    Bot(#[from] BotError),

    #[error(transparent)]
    HttpResponseBack(#[from] HttpErrorResponse),

    #[error(transparent)]
    ScyllaDatabase(#[from] ScyllaError),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheusError),

    #[error("Tokio join error: {0}")]
    TokioJoin(#[from] JoinError),

    #[error("Lettre transport error: {0}")]
    LettreTransport(#[from] lettreTransportError),

    #[error("Lettre address error: {0}")]
    LettreAddress(#[from] AddressError),

    #[error("Lettre general error: {0}")]
    LettreGeneral(#[from] lettreGeneralError),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("SerdeJson error: {0}")]
    ToJson(#[from] serdeJsonError),

    #[error("ParseInt error: {0}")]
    ToInt(#[from] ParseIntError),

    #[error("MeiliError error: {0}")]
    MeiliError(#[from] meiliError),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhowError),

    #[error("TokioCron error: {0}")]
    TokioCron(#[from] JobSchedulerError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwestError),

    #[error("Strum error: {0}")]
    Strum(#[from] strumError),
}

impl IntoResponse for HttpErrorResponse {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            HttpErrorResponse::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            HttpErrorResponse::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        (status, message).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::HttpResponseBack(inner) => inner.into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
        }
    }
}
