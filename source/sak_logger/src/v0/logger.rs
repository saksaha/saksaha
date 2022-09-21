use crate::v0::formatters::{
    ConsoleLogFormatter, FileLogFormatter, FileWriter, TestLogFormatter,
    TestLogVisitor,
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
        log_root_dir: &PathBuf,
        log_dir_name: &str,
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

        let log_dir = log_root_dir.join(log_dir_name).join("logs");
        std::fs::create_dir_all(&log_dir)?;

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
        log_root_dir: &PathBuf,
        log_dir_names: &[&str],
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let mut file_writers = vec![];
        let mut guards = vec![];

        for log_dir_name in log_dir_names {
            let log_dir = log_root_dir.join(log_dir_name).join("logs");
            std::fs::create_dir_all(&log_dir)?;

            let file_appender =
                tracing_appender::rolling::daily(log_dir, file_name_prefix);

            let (non_blocking, guard) =
                tracing_appender::non_blocking(file_appender);

            file_writers.push(FileWriter {
                log_dir_name: log_dir_name.to_string(),
                non_blocking,
            });

            guards.push(guard);
        }

        let test_log_formatter = TestLogFormatter { file_writers };

        let layer = tracing_subscriber::fmt::layer()
            .event_format(test_log_formatter)
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
