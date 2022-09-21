use crate::v0::formatters::{ConsoleLogFormatter, FileLogFormatter};
use crate::v0::utils;
use crate::LoggerError;
use colored::Colorize;
use std::path::PathBuf;
pub use tracing::{debug, error, info, trace, warn};
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    prelude::*,
    Layer,
};

pub struct SakLogger {
    _guards: Vec<WorkerGuard>,
}

impl SakLogger {
    pub fn init(
        log_dir: &PathBuf,
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let file_appender =
            tracing_appender::rolling::daily(log_dir, file_name_prefix);

        let (non_blocking, guard) =
            tracing_appender::non_blocking(file_appender);

        let layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::INFO)
            .boxed();

        layers.push(layer);

        let layer = tracing_subscriber::fmt::layer()
            .event_format(FileLogFormatter)
            .with_writer(non_blocking)
            .with_filter(EnvFilter::from_default_env())
            .boxed();

        layers.push(layer);

        tracing_subscriber::registry().with(layers).try_init()?;

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");

        let logger = SakLogger {
            _guards: vec![guard],
        };

        Ok(logger)
    }

    pub fn init_for_test(
        log_dir: &PathBuf,
        file_name_prefixes: &[&str],
    ) -> Result<SakLogger, LoggerError> {
        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let mut guards = vec![];

        for file_name_prefix in file_name_prefixes {
            let file_appender =
                tracing_appender::rolling::daily(log_dir, file_name_prefix);

            let (non_blocking, guard) =
                tracing_appender::non_blocking(file_appender);

            // let layer = tracing_subscriber::fmt::layer()
            //     .event_format(FileLogFormatter)
            //     .with_writer(non_blocking)
            //     .with_filter(EnvFilter::from_default_env())
            //     .boxed();

            // layers.push(layer);
            guards.push(guard);
        }

        let layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::INFO)
            .boxed();

        layers.push(layer);

        tracing_subscriber::registry().with(layers).try_init()?;

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");

        let logger = SakLogger { _guards: guards };

        Ok(logger)
    }
}
