use env_logger::{Builder, Env};
use std::io::Write;

fn init_logger() {
    let env = Env::default().filter("LOG_LEVEL").write_style("LOG_STYLE");

    Builder::from_env(env)
        .format(|buf, record| {
            let timestamp = buf.timestamp_millis();
            let style = buf.default_level_style(record.level());
            let level = format!("{:>width$}", record.level(), width = 5);

            writeln!(
                buf,
                "{}  {}  {}",
                timestamp,
                style.value(level),
                record.args(),
            )
        })
        .init();
}

pub fn init() {
    init_logger();

    tinfo!("logger", "logger is initialized");
}

#[macro_export]
macro_rules! tinfo {
    ($tag: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::info!("{}  {}", t, $str_format);
        }
    };

    ($tag: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::info!("{}  {}", t, format_args!($str_format, $($arg)*));
        };
    }
}

#[macro_export]
macro_rules! tdebug {
    ($tag: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::debug!("{}  {}", t, $str_format);
        }
    };

    ($tag: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::debug!("{}  {}", t, format_args!($str_format, $($arg)*));
        };
    }
}

#[macro_export]
macro_rules! terr {
    ($tag: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::error!("{}  {}", t, $str_format);
        }
    };

    ($tag: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::error!("{} {}", t, format_args!($str_format, $($arg)*));
        };
    }
}

#[macro_export]
macro_rules! twarn {
    ($tag: expr, $str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::warn!("{}  {}", t, $str_format);
        }
    };

    ($tag: expr, $str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            let t = std::format!("{:>w$}", $tag, w = 16);
            log::warn!("{}  {}", t, format_args!($str_format, $($arg)*));
        };
    }
}
