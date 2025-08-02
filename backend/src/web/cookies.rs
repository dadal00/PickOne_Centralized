use super::{
    models::{RedisAction, WebsitePath},
    utilities::get_website_path,
};
use crate::{AppError, AppState, microservices::redis::remove_id};
use axum::http::{
    HeaderValue,
    header::{HeaderMap, SET_COOKIE},
};
use axum_extra::extract::CookieJar;
use cookie::{
    Cookie, CookieBuilder, CookieJar as cookieCookieJar, SameSite::Strict, time::Duration,
};
use once_cell::sync::Lazy;
use std::sync::Arc;

static COOKIES_TO_CLEAR: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        RedisAction::Session.as_ref(),
        RedisAction::Forgot.as_ref(),
        RedisAction::Update.as_ref(),
        RedisAction::Auth.as_ref(),
    ]
});

pub static CLEARED_COOKIES_SWAP: Lazy<cookieCookieJar> =
    Lazy::new(|| cleared_cookies_for(WebsitePath::BoilerSwap));

pub fn cleared_cookies_for(website_path: WebsitePath) -> cookieCookieJar {
    let mut jar = cookieCookieJar::new();

    for &old_cookie in COOKIES_TO_CLEAR.iter() {
        let expired = build_cookie(old_cookie, "0", website_path.as_ref(), 0);

        jar.add(expired);
    }

    jar
}

pub fn clear_cookies(label: &str) -> HeaderMap {
    generate_cookie("", "", 0, &get_website_path(label))
}

pub async fn remove_cookie(
    state: Arc<AppState>,
    headers: &HeaderMap,
    redis_action: RedisAction,
) -> Result<(), AppError> {
    let id = get_cookie(headers, redis_action.as_ref());

    if id.is_some() {
        remove_id(
            state.clone(),
            &format!("{}:{}", redis_action.as_ref(), id.expect("is_none failed")),
        )
        .await?;
    }

    Ok(())
}

pub fn generate_cookie(
    key: &str,
    value: &str,
    ttl_seconds: i64,
    website_path: &WebsitePath,
) -> HeaderMap {
    let mut jar = get_cleared_cookies(website_path);

    let new_cookie = build_cookie(key, value, website_path.as_ref(), ttl_seconds);

    jar.add(new_cookie);

    let mut headers = HeaderMap::new();

    for cookie in jar.delta() {
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    headers
}

pub fn get_cookie(headers: &HeaderMap, key: &str) -> Option<String> {
    CookieJar::from_headers(headers)
        .get(key)
        .map(|cookie| cookie.value().to_string())
}

fn build_cookie(
    key: &str,
    value: &str,
    website_path: &str,
    ttl_seconds: i64,
) -> CookieBuilder<'static> {
    Cookie::build((key.to_owned(), value.to_owned()))
        .path(format!("/{website_path}"))
        .http_only(true)
        .secure(true)
        .same_site(Strict)
        .max_age(Duration::seconds(ttl_seconds))
}

fn get_cleared_cookies(website_path: &WebsitePath) -> cookieCookieJar {
    match website_path {
        WebsitePath::BoilerSwap => CLEARED_COOKIES_SWAP.clone(),
        WebsitePath::Photos => {
            panic!("Photos has no cookies")
        }
        WebsitePath::Home => {
            panic!("Home has no cookies")
        }
        WebsitePath::Housing => {
            panic!("Housing has no cookies")
        }
    }
}
