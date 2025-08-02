use super::models::{RedisAction, VerifiedTokenResult, WebsitePath};
use crate::{AppError, AppState, WebsiteRoute, microservices::redis::try_get};
use axum::{extract::Request, http::header::HeaderMap};
use sha2::{Digest, Sha256};
use std::{net::IpAddr, sync::Arc};

pub async fn clear_all_keys(
    state: Arc<AppState>,
    website_path: &str,
    keys: &[&str],
    email: &str,
) -> Result<(), AppError> {
    let mut pipe = redis::pipe();

    for key in keys {
        pipe.del(format!("{website_path}:{key}:{email}")).ignore();
    }

    pipe.query_async::<()>(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn format_verified_result(
    state: Arc<AppState>,
    website_path: &WebsitePath,
    redis_action: RedisAction,
    id: String,
) -> Result<Option<VerifiedTokenResult>, AppError> {
    if let Some(serialized_account) = try_get(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
            redis_action.as_ref(),
            &id
        ),
    )
    .await?
    {
        return Ok(Some(VerifiedTokenResult {
            serialized_account: Some(serialized_account),
            redis_action,
            id,
        }));
    }

    Ok(None)
}

pub fn label_request(request: &mut Request, website_path: WebsitePath) {
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
