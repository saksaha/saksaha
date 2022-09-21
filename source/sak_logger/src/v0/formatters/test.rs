use crate::v0::utils;
use crate::{LoggerError, RUST_LOG_ENV};
use chrono::Local;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;
pub use tracing::{debug, error, info, trace, warn};
use tracing::{Event, Metadata, Subscriber};
use tracing_appender::non_blocking::NonBlocking;
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::fmt::{
    format, FmtContext, FormatEvent, FormatFields, FormattedFields,
};
use tracing_subscriber::layer::{Context, Filter};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    Layer,
};

pub struct TestLogFormatter {
    pub log_dir_name: String,
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

        let now = Local::now().format("%y-%m-%d %H:%M:%S");

        let mut visitor = TestLogVisitor {
            should_log: false,
            log_dir_name: &self.log_dir_name,
        };

        event.record(&mut visitor);

        if !visitor.should_log {
            return write!(writer, "");
        }

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

struct TestLogVisitor<'a> {
    pub should_log: bool,
    pub log_dir_name: &'a str,
}

impl<'a> tracing::field::Visit for TestLogVisitor<'a> {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("field={} value={}", field.name(), value);

        if field.name() == "public_key" {
            if self.log_dir_name == value {
                println!("333 field={} value={}", field.name(), value);
                self.should_log = true;
            }
        }
    }

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        _value: &(dyn std::error::Error + 'static),
    ) {
    }

    fn record_debug(
        &mut self,
        _field: &tracing::field::Field,
        _value: &dyn std::fmt::Debug,
    ) {
    }
}
