use anyhow::Result;
use log::{debug, error, info, trace, warn};
use std::env;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::prelude::*;

pub fn init() -> Result<Option<tracing_appender::non_blocking::WorkerGuard>> {
    let filter = tracing_subscriber::EnvFilter::from_env("LOG_LEVEL");
    let stdout_layer = tracing_subscriber::fmt::layer();

    let builder = tracing_subscriber::registry::Registry::default()
        .with(filter)
        .with(stdout_layer);

    // also log to file if one is provided via env
    let guard = if let Ok(log_file) = env::var("LOG_FILE") {
        let file_appender = tracing_appender::rolling::hourly("./log", log_file);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        builder
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
            .init();
        Some(guard)
    } else {
        builder.init();
        None
    };

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");

    Ok(guard)
}

pub fn create_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
