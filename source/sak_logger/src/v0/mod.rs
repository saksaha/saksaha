mod logger;
mod macros;

use env_logger::{Builder, Env, Logger};
pub use logger::*;
use std::cmp::min;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format, FmtContext, FormatEvent, FormatFields, FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

use std::fs::File;
use std::io;
use tracing_subscriber;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    prelude::*,
    Layer,
};

pub type LoggerError = Box<dyn std::error::Error + Send + Sync>;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub const RUST_LOG_ENV: &str = "sak_=debug,saksaha_=debug,hyper_=debug";

// pub const RUST_LOG_ENV: &str = "
//     sak_=debug,
//     saksaha_=debug,
//     hyper_=debug,
//     wallet_=debug,
// ";

pub fn init(is_test: bool) -> Result<(), String> {
    if IS_INITIALIZED.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(format!("Logger is already initialized"));
    }

    {
        let rust_log = match std::env::var("RUST_LOG") {
            Ok(l) => l,
            Err(_) => {
                println!(
                    "RUST_LOG is not given. This is probably not what you \
                have wanted. Some logs might be dismissed"
                );

                "RUST_LOG_NOT_GIVEN".to_string()
            }
        };

        println!("[logger] Initializing logger, RUST_LOG: {}", rust_log);

        let logger = build_logger(is_test);

        let max_level = logger.filter();
        let res = log::set_boxed_logger(Box::new(logger));

        match res {
            Ok(_) => {
                log::set_max_level(max_level);

                IS_INITIALIZED
                    .store(true, std::sync::atomic::Ordering::Relaxed);

                log::info!("Logger initialized");

                return Ok(());
            }
            Err(err) => {
                return Err(format!(
                    "Logger might have been initialized, err: {}",
                    err
                ));
            }
        }
    }
}

fn build_logger(is_test: bool) -> Logger {
    let env = Env::default().write_style("LOG_STYLE");

    Builder::from_env(env)
        .is_test(is_test)
        .format(|buf, record| {
            let timestamp = buf.timestamp_millis();
            let style = buf.default_level_style(record.level());
            let level = format!("{:>width$}", record.level(), width = 5);

            let target = {
                let target = record.metadata().target();
                let split: Vec<&str> = target.split("::").collect();
                let len = split.len();

                if len >= 2 {
                    let seg1 = split[len - 1];
                    let seg2 = split[len - 2];
                    format!(
                        "{}/{}",
                        &seg2[0..min(seg2.len(), 10)],
                        &seg1[0..min(seg1.len(), 10)]
                    )
                } else {
                    format!("{}", split[0])
                }
            };

            writeln!(
                buf,
                "{} {} {:21} {}",
                timestamp,
                style.value(level),
                target,
                record.args(),
            )
        })
        .build()
}

pub fn setup_logger2(log_dir: &PathBuf) -> Result<(), LoggerError> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    let a = std::env::var("RUST_LOG");
    println!("rust_log, {:?}", a);

    let mut layers = Vec::new();

    let log_file_path = log_dir.join("file.log");

    let file = std::fs::File::create(&log_file_path).unwrap();

    let layer = tracing_subscriber::fmt::layer()
        .event_format(MyFormatter)
        .with_filter(EnvFilter::from_default_env())
        .boxed();

    layers.push(layer);

    let layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_target(true)
        .event_format(MyFormatter)
        .with_writer(file)
        .with_filter(EnvFilter::from_default_env())
        .boxed();

    layers.push(layer);

    tracing_subscriber::registry().with(layers).try_init()?;

    tracing::info!("info 2");
    tracing::warn!("warn 2");
    tracing::error!("error 2");

    Ok(())
}

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
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
        write!(
            &mut writer,
            "123123 {} {}: ",
            metadata.level(),
            metadata.target()
        )?;

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
