use crate::v0::utils;
use crate::{LoggerError, RUST_LOG_ENV};
use chrono::Local;
use colored::Colorize;
use std::path::PathBuf;
pub use tracing::{debug, error, info, trace, warn};
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::NonBlocking;
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

pub struct TestLogFormatter {
    pub file_writers: Vec<FileWriter>,
}

pub struct FileWriter {
    pub file_name_prefix: String,
    pub non_blocking: NonBlocking,
}

impl<S, N> FormatEvent<S, N> for TestLogFormatter
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

        event.record(&mut PrintlnVisitor);

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

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(
        &mut self,
        field: &tracing::field::Field,
        value: &dyn std::fmt::Debug,
    ) {
        println!("  field={} value={:?}", field.name(), value)
    }
}
