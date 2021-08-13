use crate::StdErr;
use log::{debug, error, info, trace, warn};
use std::env;
use tracing_subscriber::prelude::*;

pub fn init() -> Result<(), StdErr> {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    let filter = tracing_subscriber::EnvFilter::from_default_env().add_directive(log_level.into());
    let stdout_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    let builder = tracing_subscriber::registry::Registry::default()
        .with(filter)
        .with(stdout_layer);

    // also log to file if one is provided via env
    if let Ok(log_file) = env::var("LOG_FILE") {
        // TODO: check file
        let file_appender = tracing_appender::rolling::hourly("./log", log_file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        builder
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
            .init();
    } else {
        builder.init();
    }

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");

    Ok(())
}
