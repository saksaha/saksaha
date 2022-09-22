use crate::v0::formatters::{ConsoleLogFormatter, FileLogFormatter, TestLogFormatter};
use crate::v0::utils;
use crate::LoggerError;
use colored::Colorize;
use std::path::Path;
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
    pub fn init<P: AsRef<Path>>(
        log_root_dir: P,
        log_dir_name: &str,
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        println!(
            "sak_logger: initializing, log_root_dir: {:?}, \
            log_dir_name: {}, file_name_prefix: {}",
            log_root_dir.as_ref(),
            log_dir_name,
            file_name_prefix
        );

        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let console_log_layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::INFO)
            .boxed();

        layers.push(console_log_layer);

        let guard = {
            let log_dir = log_root_dir.as_ref().join(log_dir_name).join("logs");
            std::fs::create_dir_all(&log_dir)?;

            let file_appender = tracing_appender::rolling::daily(&log_dir, file_name_prefix);

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let file_log_layer = tracing_subscriber::fmt::layer()
                .event_format(FileLogFormatter)
                .with_writer(non_blocking)
                .with_filter(EnvFilter::from_default_env())
                .boxed();

            layers.push(file_log_layer);

            println!(
                "sak_logger: log will be persisted in dir: {}",
                log_dir.to_string_lossy().yellow(),
            );

            guard
        };

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

    pub fn init_test_console() -> Result<SakLogger, LoggerError> {
        println!("Initializing sak_logger for test (console)");

        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::DEBUG)
            .boxed();

        layers.push(layer);

        tracing_subscriber::registry().with(layers).try_init()?;

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");

        let logger = SakLogger { _guards: vec![] };

        Ok(logger)
    }

    pub fn init_test_persisted<P: AsRef<Path>>(
        log_root_dir: P,
        log_dir_names: &[&str],
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        println!("Initializing sak_logger for test (persisted)");

        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::DEBUG)
            .boxed();

        layers.push(layer);

        let mut _guards = vec![];

        for log_dir_name in log_dir_names {
            let log_dir = log_root_dir.as_ref().join(log_dir_name).join("logs");
            std::fs::create_dir_all(&log_dir)?;

            let file_appender = tracing_appender::rolling::daily(&log_dir, file_name_prefix);

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            println!("sak_logger is writing to log_dir: {:?}", log_dir);

            let test_log_formatter = TestLogFormatter {
                log_dir_name: log_dir_name.to_string(),
            };

            let layer = tracing_subscriber::fmt::layer()
                .event_format(test_log_formatter)
                .with_writer(non_blocking)
                .with_filter(EnvFilter::from_default_env())
                .boxed();

            layers.push(layer);
            _guards.push(guard);

            println!(
                "sak_logger for test, adding layer, log_dir_name: {}",
                log_dir_name
            );
        }

        tracing_subscriber::registry().with(layers).try_init()?;

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");

        let logger = SakLogger { _guards };

        Ok(logger)
    }
}
