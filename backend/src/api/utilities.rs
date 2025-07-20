use super::{
    microservices::redis::{incr_visitors, try_get},
    models::{RedisAction, VerifiedTokenResult, WebsitePath},
};
use crate::{AppError, AppState, WebsiteRoute};
use axum::{extract::Request, http::header::HeaderMap};
use sha2::{Digest, Sha256};
use std::{net::IpAddr, sync::Arc};

pub fn get_hashed_ip(headers: &HeaderMap, direct_ip: IpAddr) -> String {
    let ip = headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next().map(|s| s.trim().to_string()))
        })
        .unwrap_or_else(|| direct_ip.to_string());

    let mut hasher = Sha256::new();

    hasher.update(ip.as_bytes());

    format!("{:x}", hasher.finalize())
}

pub fn get_key(redis_action: RedisAction, hashed_ip: &str) -> String {
    format!("{}:{}", redis_action.as_ref(), hashed_ip)
}

pub fn convert_i8_to_u8(payload: &i8) -> u8 {
    payload.checked_abs().unwrap_or(0) as u8
}

pub async fn format_verified_result(
    state: Arc<AppState>,
    website_path: &WebsitePath,
    redis_action: RedisAction,
    id: String,
) -> Result<Option<VerifiedTokenResult>, AppError> {
    if redis_action == RedisAction::Session {
        if let Some(result) = try_get(
            state.clone(),
            &format!(
                "{}:{}:{}",
                WebsitePath::BoilerSwap.as_ref(),
                RedisAction::Session.as_ref(),
                &id
            ),
        )
        .await?
        {
            return Ok(Some(VerifiedTokenResult {
                serialized_account: Some(result),
                redis_action: RedisAction::Session,
                id,
            }));
        }
        return Ok(None);
    }

    Ok(Some(VerifiedTokenResult {
        serialized_account: try_get(
            state.clone(),
            &format!("{}:{}:{}", website_path.as_ref(), redis_action.as_ref(), id),
        )
        .await?,
        redis_action,
        id,
    }))
}

pub async fn check_path(
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

fn label_request(request: &mut Request, website_path: WebsitePath) {
    match request.uri().path() {
        path if path
            == format!(
                "/{}/{}/{}",
                website_path.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Authenticate.as_ref()
            )
            || path
                == format!(
                    "/{}/{}/{}",
                    website_path.as_ref(),
                    WebsiteRoute::Api.as_ref(),
                    WebsiteRoute::Verify.as_ref()
                )
            || path
                == format!(
                    "/{}/{}/{}",
                    website_path.as_ref(),
                    WebsiteRoute::Api.as_ref(),
                    WebsiteRoute::Delete.as_ref()
                )
            || path
                == format!(
                    "/{}/{}/{}",
                    website_path.as_ref(),
                    WebsiteRoute::Api.as_ref(),
                    WebsiteRoute::Forgot.as_ref()
                )
            || path
                == format!(
                    "/{}/{}/{}",
                    website_path.as_ref(),
                    WebsiteRoute::Api.as_ref(),
                    WebsiteRoute::Resend.as_ref()
                ) =>
        {
            request
                .extensions_mut()
                .insert(website_path.as_ref().to_string());
        }
        _ => {}
    }
}

pub fn get_website_path(label: &str) -> WebsitePath {
    match label {
        _ if label == WebsitePath::BoilerSwap.as_ref() => WebsitePath::BoilerSwap,
        _ => {
            panic!("Added connection invalid")
        }
    }
}
