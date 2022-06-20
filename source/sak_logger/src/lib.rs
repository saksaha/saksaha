use env_logger::{Builder, Env};
use log::info;
use log::Record;
use std::cmp::min;
use std::io::Write;

fn init_logger(is_test: bool) {
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
        .init();
}

pub fn init(is_test: bool) {
    init_logger(is_test);

    info!("Logger is initialized");
}

#[macro_export]
macro_rules! tag {
    ($tag1: expr, $tag2: expr) => {
        std::format!("[{:>w1$}] [{:>w2$}]", $tag1, $tag2, w1 = 16, w2 = 10)
    };
}

#[macro_export]
macro_rules! tinfo {
    ($tag1: expr, $tag2: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::info!("{} {}", t, $str_format);
        }
    };

    ($tag1: expr, $tag2: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::info!("{} {}", t, format_args!($str_format, $($arg)*));
        };
    };
}

#[macro_export]
macro_rules! tdebug {
    ($tag1: expr, $tag2: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::debug!("{} {}", t, $str_format);
        };
    };

    ($tag1: expr, $tag2: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::debug!("{} {}", t, format_args!($str_format, $($arg)*));
        };
    };
}

#[macro_export]
macro_rules! terr {
    ($tag1: expr, $tag2: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::error!("{} {}", t, $str_format);
        };
    };

    ($tag1: expr, $tag2: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::error!("{} {}", t, format_args!($str_format, $($arg)*));
        };
    }
}

#[macro_export]
macro_rules! twarn {
    ($tag1: expr, $tag2: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::warn!("{} {}", t, $str_format);
        };
    };

    ($tag1: expr, $tag2: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = $crate::tag!($tag1, $tag2);
            log::warn!("{} {}", t, format_args!($str_format, $($arg)*));
        };
    };
}
