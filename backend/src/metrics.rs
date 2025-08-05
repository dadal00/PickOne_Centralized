use crate::AppError;
use prometheus::{Encoder, Registry, TextEncoder, register_int_counter};

pub async fn metrics_handler() -> Result<String, AppError> {
    let registry = Registry::new();
    let encoder = TextEncoder::new();

    let dummy = register_int_counter!("dummy", "Dummy metric").expect("Can't create dummy metric");
    registry.register(Box::new(dummy.clone())).unwrap();

    let metric_families = registry.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}
