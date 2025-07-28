use crate::{
    AppError, AppState,
    microservices::redis::try_get,
    web::models::{RedisAction, VisitorPayload, WebsitePath},
};
use axum::{Json, extract::State};
use once_cell::sync::Lazy;
use prometheus::{Encoder, IntCounter, IntGauge, Registry, TextEncoder};
use redis::{AsyncTypedCommands, Script, aio::ConnectionManager};
use std::sync::Arc;
use strum_macros::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, PartialEq)]
pub enum RedisMetricAction {
    #[strum(serialize = "visitors")]
    Visitors,

    #[strum(serialize = "items")]
    Items,
}

static DECR_METRIC_SCRIPT: Lazy<Script> = Lazy::new(|| {
    Script::new(
        r#"
        local attempts = redis.call("DECR", KEYS[1])
        if attempts < 0 then
            redis.call("SET", KEYS[1], 0)
        end
    "#,
    )
});

pub async fn metrics_handler(State(state): State<Arc<AppState>>) -> Result<String, AppError> {
    let registry = Registry::new();
    let encoder = TextEncoder::new();

    let swap_visitors = IntCounter::new("swap_visitors", "Total visitors on BoilerSwap").unwrap();
    let swap_items = IntGauge::new("swap_items", "Total items on BoilerSwap").unwrap();
    let bot_visitors = IntCounter::new("bot_visitors", "Total visitors on BoilerCuts").unwrap();
    let home_visitors = IntCounter::new("home_visitors", "Total visitors on Home").unwrap();

    pull_metric(
        state.clone(),
        WebsitePath::BoilerSwap,
        RedisMetricAction::Visitors,
        &swap_visitors,
    )
    .await?;
    pull_metric(
        state.clone(),
        WebsitePath::Photos,
        RedisMetricAction::Visitors,
        &bot_visitors,
    )
    .await?;
    pull_metric(
        state.clone(),
        WebsitePath::Home,
        RedisMetricAction::Visitors,
        &home_visitors,
    )
    .await?;
    set_metric(
        state.clone(),
        WebsitePath::BoilerSwap,
        RedisMetricAction::Items,
        &swap_items,
    )
    .await?;

    registry.register(Box::new(swap_visitors))?;
    registry.register(Box::new(bot_visitors))?;
    registry.register(Box::new(home_visitors))?;

    let metric_families = registry.gather();

    let mut buffer = vec![];

    encoder.encode(&metric_families, &mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

pub async fn get_visitors_payload(
    state: Arc<AppState>,
) -> Result<Json<Vec<VisitorPayload>>, AppError> {
    Ok(Json(vec![
        VisitorPayload {
            website: WebsitePath::BoilerSwap.as_ref().to_string(),
            visitors: get_metric(
                state.clone(),
                WebsitePath::BoilerSwap,
                RedisMetricAction::Visitors,
            )
            .await?,
        },
        VisitorPayload {
            website: WebsitePath::Photos.as_ref().to_string(),
            visitors: get_metric(
                state.clone(),
                WebsitePath::Photos,
                RedisMetricAction::Visitors,
            )
            .await?,
        },
        VisitorPayload {
            website: WebsitePath::Home.as_ref().to_string(),
            visitors: get_metric(
                state.clone(),
                WebsitePath::Home,
                RedisMetricAction::Visitors,
            )
            .await?,
        },
    ]))
}

async fn pull_metric(
    state: Arc<AppState>,
    website_path: WebsitePath,
    metric_action: RedisMetricAction,
    counter: &IntCounter,
) -> Result<(), AppError> {
    counter.inc_by(get_metric(state.clone(), website_path, metric_action).await?);

    Ok(())
}

async fn set_metric(
    state: Arc<AppState>,
    website_path: WebsitePath,
    metric_action: RedisMetricAction,
    gauge: &IntGauge,
) -> Result<(), AppError> {
    gauge.set(
        get_metric(state.clone(), website_path, metric_action)
            .await?
            .try_into()
            .unwrap(),
    );

    Ok(())
}

async fn get_metric(
    state: Arc<AppState>,
    website_path: WebsitePath,
    metric_action: RedisMetricAction,
) -> Result<u64, AppError> {
    Ok(try_get(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
            RedisAction::Metric.as_ref(),
            metric_action.as_ref()
        ),
    )
    .await?
    .and_then(|s| s.parse::<u64>().ok())
    .unwrap_or(0))
}

pub async fn incr_metric(state: Arc<AppState>, key: &str) -> Result<(), AppError> {
    state.redis_connection_manager.clone().incr(key, 1).await?;

    Ok(())
}

pub async fn incr_visitors(
    state: Arc<AppState>,
    website_path: WebsitePath,
) -> Result<(), AppError> {
    incr_metric(
        state.clone(),
        &format!(
            "{}:{}:{}",
            website_path.as_ref(),
            RedisAction::Metric.as_ref(),
            RedisMetricAction::Visitors.as_ref()
        ),
    )
    .await?;

    Ok(())
}

pub async fn decr_metric(state: Arc<AppState>, key: &str) -> Result<(), AppError> {
    let _: () = DECR_METRIC_SCRIPT
        .key(key)
        .invoke_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn set_redis_metric(
    mut redis_connection_manager: ConnectionManager,
    key: &str,
    val: &usize,
) -> Result<(), AppError> {
    redis_connection_manager.set(key, val).await?;

    Ok(())
}
