use crate::v0::formatters::{ConsoleLogFormatter, FileLogFormatter, TestLogFormatter};
use crate::v0::global::LOGGER;
use crate::v0::utils;
use crate::LoggerError;
use colored::Colorize;
use once_cell::sync::OnceCell;
use std::path::Path;
pub use tracing::{debug, error, info, trace, warn};
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    prelude::*,
    Layer,
};

const LOG_FILE_PREFIX: &str = "saksaha.log";

#[derive(Debug)]
pub enum LoggerType {
    DEFAULT,
    TEST,
}

pub struct SakLogger {
    _guards: Vec<WorkerGuard>,
    ty: LoggerType,
}

impl SakLogger {
    pub fn init<P: AsRef<Path>>(log_root_dir: P, log_dir_name: &str) -> Result<(), LoggerError> {
        if let Some(l) = LOGGER.get() {
            println!(
                "{}: Logger is already initialized, type: {:?}",
                "warn".yellow().bold(),
                l.ty
            );

            return Ok(());
        }

        let rust_log_env = utils::set_rust_log_env();

        println!(
            "\n{}
    {}: {:?}
    {}: {:?}
    {}: {:?}
    {}: {:?}",
            "Initializing sak_logger".magenta().bold(),
            "Log root dir".cyan().bold(),
            log_root_dir.as_ref(),
            "Log dir name".cyan().bold(),
            log_dir_name,
            "File name prefix".cyan().bold(),
            LOG_FILE_PREFIX,
            "RUST_LOG_ENV".cyan().bold(),
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

            let file_appender = tracing_appender::rolling::daily(&log_dir, LOG_FILE_PREFIX);

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let file_log_layer = tracing_subscriber::fmt::layer()
                .event_format(FileLogFormatter)
                .with_writer(non_blocking)
                .with_filter(EnvFilter::from_default_env())
                .boxed();

            layers.push(file_log_layer);

            println!(
                "File logger is attached. Log files will be periodically rotated.
    {}: {}",
                "Log dir".cyan().bold(),
                log_dir.to_string_lossy(),
            );

            guard
        };

        println!(
            "    {}\n",
            "Following log invocation will be handled by global logger"
                .magenta()
                .bold()
        );

        tracing_subscriber::registry().with(layers).try_init()?;

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");
        tracing::trace!("sak_logger is initialized");

        let logger = SakLogger {
            _guards: vec![guard],
            ty: LoggerType::DEFAULT,
        };

        let _ = LOGGER.set(logger);
        Ok(())
    }

    pub fn init_test_console() -> Result<(), LoggerError> {
        if let Some(l) = LOGGER.get() {
            println!(
                "{}: Logger is already initialized, type: {:?}",
                "warn".yellow().bold(),
                l.ty
            );

            return Ok(());
        }

        println!("Initializing sak_logger for test (console)");

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
                println!(
                    "{}: Test console logger is already initialized, err: {}",
                    "warn".yellow().bold(),
                    err,
                );
            }
        };

        tracing::info!("sak_logger is initialized");
        tracing::warn!("sak_logger is initialized");
        tracing::error!("sak_logger is initialized");
        tracing::debug!("sak_logger is initialized");

        let logger = SakLogger {
            _guards: vec![],
            ty: LoggerType::TEST,
        };

        let _ = LOGGER.set(logger);
        Ok(())
    }

    pub fn init_test_persisted<P>(log_root_dir: P) -> Result<(), LoggerError>
    where
        P: AsRef<Path>,
    {
        if let Some(l) = LOGGER.get() {
            println!(
                "{}: Logger is already initialized, type: {:?}",
                "warn".yellow().bold(),
                l.ty
            );
            return Ok(());
        }

        println!("Initializing sak_logger for test (persisted)");

        utils::set_rust_log_env();

        let mut layers = Vec::new();

        let console_log_layer = tracing_subscriber::fmt::layer()
            .event_format(ConsoleLogFormatter)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::DEBUG)
            .boxed();

        layers.push(console_log_layer);

        let test_log_formatter = TestLogFormatter {};

        let log_dir = log_root_dir.as_ref().join("test/logs");
        let file_appender = tracing_appender::rolling::daily(&log_dir, LOG_FILE_PREFIX);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let layer = tracing_subscriber::fmt::layer()
            .event_format(test_log_formatter)
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
            ty: LoggerType::TEST,
        };

        let _ = LOGGER.set(logger);
        Ok(())
    }
}
