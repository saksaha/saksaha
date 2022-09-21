use crate::v0::formatters::{
    ConsoleLogFormatter, FileLogFormatter, FileWriter, TestLogFormatter,
};
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

        let console_log_layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::INFO)
            .boxed();

        layers.push(console_log_layer);

        let file_appender =
            tracing_appender::rolling::daily(log_dir, file_name_prefix);

        let (non_blocking, guard) =
            tracing_appender::non_blocking(file_appender);

        let file_log_layer = tracing_subscriber::fmt::layer()
            .event_format(FileLogFormatter)
            .with_writer(non_blocking)
            .with_filter(EnvFilter::from_default_env())
            .boxed();

        layers.push(file_log_layer);

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
        log_dirs: &[PathBuf],
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let mut file_writers = vec![];
        let mut guards = vec![];

        for log_dir in log_dirs {
            let file_appender =
                tracing_appender::rolling::daily(log_dir, file_name_prefix);

            let (non_blocking, guard) =
                tracing_appender::non_blocking(file_appender);

            file_writers.push(FileWriter {
                file_name_prefix: file_name_prefix.to_string(),
                non_blocking,
            });

            guards.push(guard);
        }

        let layer = tracing_subscriber::fmt::layer()
            .event_format(TestLogFormatter { file_writers })
            .with_filter(EnvFilter::from_default_env())
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
