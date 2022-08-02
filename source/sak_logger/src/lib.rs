mod macros;

use env_logger::Logger;
use env_logger::{Builder, Env};
use log::info;
use log::Record;
use std::cmp::min;
use std::io::Write;
use std::sync::atomic::AtomicBool;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

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
