#[macro_export]
macro_rules! log {
    ($str_format: expr) => {
        #[cfg(debug_assertions)]
        {
            std::println!("[ci] {}", std::format_args!($str_format));
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            std::println!("[ci] {}", format_args!($str_format, $($arg)*));
        }
    };
}
