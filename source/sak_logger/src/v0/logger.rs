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

static LOGGER: std::sync::Once = std::sync::Once::new();

pub struct SakLogger {
    _guards: Vec<WorkerGuard>,
}

impl SakLogger {
    pub fn init<P: AsRef<Path>>(
        log_root_dir: P,
        log_dir_name: &str,
        file_name_prefix: &str,
    ) -> Result<SakLogger, LoggerError> {
        let rust_log_env = utils::set_rust_log_env();

        println!(
            "\n\n>> Initializing sak_logger\n\
            {}: {:?}\n{}: {}\n{}: {}\n{}: {}",
            "    Log root dir".cyan(),
            log_root_dir.as_ref(),
            "    Log dir name".cyan(),
            log_dir_name,
            "    File name prefix".cyan(),
            file_name_prefix,
            "    RUST_LOG_ENV".cyan(),
            rust_log_env,
        );

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
                "\nFile logger is attached. Log files will be periodically \
                rotated).\n{}: {}",
                "    Log dir".cyan(),
                log_dir.to_string_lossy(),
            );

            guard
        };

        tracing_subscriber::registry().with(layers).try_init()?;

        println!("\n\n");
        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");
        tracing::trace!("sak_logger is initialized");

        let logger = SakLogger {
            _guards: vec![guard],
        };

        Ok(logger)
    }

    pub fn init_test_console() -> Result<SakLogger, LoggerError> {
        println!("\nInitializing sak_logger for test (console)");

        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::DEBUG)
            .boxed();

        layers.push(layer);

        match tracing_subscriber::registry().with(layers).try_init() {
            Ok(_) => {}
            Err(err) => {
                println!("Test console logger is already initialized");
            }
        };

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
