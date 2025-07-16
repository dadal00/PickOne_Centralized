use crate::{
    AppError, AppState,
    api::models::{VisitorPayload, WebsitePath},
};
use axum::{Json, extract::State};
use prometheus::{
    Encoder, Histogram, HistogramOpts, IntCounter, IntGauge, Registry, TextEncoder,
    register_int_counter, register_int_gauge,
};
use std::sync::Arc;
use tracing::debug;

#[derive(Debug)]
pub struct Metrics {
    pub swap_visitors: IntCounter,
    pub swap_products: IntGauge,
    pub bot_visitors: IntCounter,
    pub bot_image_size_bytes: Histogram,
    pub home_visitors: IntCounter,
    registry: Registry,
}

impl Default for Metrics {
    fn default() -> Self {
        let registry = Registry::new();

        let swap_visitors = register_int_counter!("swap_visitors", "Total visitors to BoilerSwap")
            .expect("Can't create swap visitors metric");

        let swap_products = register_int_gauge!("swap_products", "Total products on BoilerSwap")
            .expect("Can't create swap products metric");

        let bot_visitors = register_int_counter!("bot_visitors", "Total visitors on BoilerCuts")
            .expect("Can't create bot visitors metric");

        let bot_image_size_bytes = Histogram::with_opts(
            HistogramOpts::new("bot_image_size_bytes", "Size of uploaded images in bytes").buckets(
                vec![
                    50_000.0,
                    100_000.0,
                    250_000.0,
                    500_000.0,
                    1_000_000.0,
                    2_000_000.0,
                    3_000_000.0,
                    5_000_000.0,
                ],
            ),
        )
        .unwrap();

        let home_visitors = register_int_counter!("home_visitors", "Total visitors on Home")
            .expect("Can't create home visitors metric");

        registry.register(Box::new(swap_visitors.clone())).unwrap();
        registry.register(Box::new(swap_products.clone())).unwrap();
        registry.register(Box::new(bot_visitors.clone())).unwrap();
        registry
            .register(Box::new(bot_image_size_bytes.clone()))
            .unwrap();
        registry.register(Box::new(home_visitors.clone())).unwrap();

        Metrics {
            swap_visitors,
            swap_products,
            bot_visitors,
            bot_image_size_bytes,
            home_visitors,
            registry,
        }
    }
}

impl Metrics {
    pub fn gather(&self) -> Result<String, AppError> {
        let encoder = TextEncoder::new();

        let metric_families = self.registry.gather();

        let mut buffer = vec![];

        encoder.encode(&metric_families, &mut buffer)?;

        Ok(String::from_utf8(buffer)?)
    }
}

pub async fn metrics_handler(State(state): State<Arc<AppState>>) -> Result<String, AppError> {
    debug!("Metrics being scrapped");

    state.metrics.gather()
}

pub fn get_visitors(state: Arc<AppState>) -> Json<Vec<VisitorPayload>> {
    Json(vec![
        VisitorPayload {
            website: WebsitePath::BoilerSwap.as_ref().to_string(),
            visitors: state.metrics.swap_visitors.get(),
        },
        VisitorPayload {
            website: WebsitePath::Photos.as_ref().to_string(),
            visitors: state.metrics.bot_visitors.get(),
        },
        VisitorPayload {
            website: WebsitePath::Home.as_ref().to_string(),
            visitors: state.metrics.home_visitors.get(),
        },
    ])
}
