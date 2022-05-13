use env_logger::{Builder, Env};
use std::io::Write;

fn init_logger(is_test: bool) {
    let env = Env::default().filter("LOG_LEVEL").write_style("LOG_STYLE");

    Builder::from_env(env)
        .is_test(is_test)
        .format(|buf, record| {
            let timestamp = buf.timestamp_millis();
            let style = buf.default_level_style(record.level());
            let level = format!("{:>width$}", record.level(), width = 5);

            writeln!(
                buf,
                "{} [{}] {}",
                timestamp,
                style.value(level),
                record.args(),
            )
        })
        .init();
}

pub fn init(is_test: bool) {
    init_logger(is_test);

    tinfo!("logger", "", "Logger is initialized");
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
