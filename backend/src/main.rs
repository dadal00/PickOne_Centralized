use crate::{
    bot::{chat::start_bot, photo::photo_handler},
    error::AppError,
    metrics::metrics_handler,
    microservices::cdc::core::{ScyllaCDCParams, start_cdc},
    signals::shutdown_signal,
    state::AppState,
    web::{
        handlers::{
            api_token_check, authenticate_handler, delete_handler, forgot_handler, resend_handler,
            verify_handler,
        },
        housing::{
            cdc::start_housing_cdc,
            handlers::{post_review_handler, update_thumbs_handler},
        },
        models::{METRICS_ROUTE, RedisAction, WebsitePath, WebsiteRoute},
        swap::{cdc::start_swap_cdc, handlers::post_item_handler},
    },
};
use anyhow::Result as anyResult;
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    middleware,
    routing::{delete, get, post},
};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod bot;
mod config;
mod error;
mod metrics;
mod microservices;
mod signals;
mod state;
mod utilities;
mod web;

#[tokio::main]
async fn main() -> anyResult<()> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), // backend (target) = info (logging level)
        )
        .init();

    info!("Starting server...");

    let (state, meili_swap_future, meili_housing_future) = AppState::new().await?;

    start_bot(state.clone()).await?;

    info!("Server configuration");
    info!("rust_port = {}", state.config.server.rust_port);
    info!("svelte_url = {}", state.config.server.svelte_url);

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            origin.as_bytes() == origin_state.config.server.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST, Method::DELETE])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Authenticate.as_ref()
            ),
            post(authenticate_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Verify.as_ref()
            ),
            post(verify_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Delete.as_ref()
            ),
            delete(delete_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Forgot.as_ref()
            ),
            post(forgot_handler),
        )
        .route(
            &format!(
                "/{}/{}/post-item",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref()
            ),
            post(post_item_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Resend.as_ref()
            ),
            post(resend_handler),
        )
        .route(
            &format!(
                "/{}/{}/post-review",
                WebsitePath::Housing.as_ref(),
                WebsiteRoute::Api.as_ref(),
            ),
            post(post_review_handler),
        )
        .route(
            &format!(
                "/{}/{}/update-thumbs",
                WebsitePath::Housing.as_ref(),
                WebsiteRoute::Api.as_ref(),
            ),
            post(update_thumbs_handler),
        )
        .route(
            &format!("/{}/:id", WebsitePath::Photos.as_ref()),
            get(photo_handler),
        )
        .route(METRICS_ROUTE, get(metrics_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            api_token_check,
        ))
        .layer(cors)
        .with_state(state.clone())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("0.0.0.0:{}", state.config.server.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;

    meili_swap_future.await??;
    meili_housing_future.await??;

    let (mut swap_cdc_reader, swap_cdc_future) = start_swap_cdc(state.clone()).await?;
    let (mut housing_cdc_reader, housing_cdc_future) = start_housing_cdc(state).await?;

    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    housing_cdc_reader.stop();
    swap_cdc_reader.stop();

    housing_cdc_future.await?;
    swap_cdc_future.await
}
