use crate::v0::utils;
use crate::{LoggerError, RUST_LOG_ENV};
use chrono::Local;
use colored::Colorize;
use std::path::PathBuf;
pub use tracing::{debug, error, info, trace, warn};
use tracing::{Event, Subscriber};
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;
use tracing_subscriber::fmt::{
    format, FmtContext, FormatEvent, FormatFields, FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;
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

struct ConsoleLogFormatter;

impl<S, N> FormatEvent<S, N> for ConsoleLogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();

        let now = Local::now().format("%y-%m-%d %H:%M:%S");

        let level = match metadata.level().as_str() {
            "INFO" => "INFO".green(),
            "WARN" => "WARN".yellow(),
            "ERROR" => "ERROR".red(),
            "DEBUG" => "DEBUG".blue(),
            _ => "".green(),
        };

        let target = metadata.target();
        let target_len = target.len();
        let target = if target_len > 16 {
            &target[target_len - 16..target_len]
        } else {
            &target
        };

        write!(&mut writer, "{} {:>5} {:>16}: ", now, level, target,)?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

struct FileLogFormatter;

impl<S, N> FormatEvent<S, N> for FileLogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        let now = Local::now().format("%y-%m-%d %H:%M:%S");

        write!(
            &mut writer,
            "{} {:>5} {}: ",
            now,
            metadata.level(),
            metadata.target()
        )?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
