pub const CI_LOGGER: &str = "saksaha ci";

#[macro_export]
macro_rules! logln {
    ($str_format: expr) => {
        #[cfg(debug_assertions)]
        {
            std::println!("{}: {}", $crate::CI_LOGGER, std::format_args!($str_format));
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            std::println!("{}: {}", $crate::CI_LOGGER, format_args!($str_format, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log {
    ($str_format: expr) => {
        #[cfg(debug_assertions)]
        {
            std::print!("{}: {}", $crate::CI_LOGGER, std::format_args!($str_format));
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            std::print!("{}: {}", $crate::CI_LOGGER, format_args!($str_format, $($arg)*));
        }
    };
}
