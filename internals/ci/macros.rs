#[macro_export]
macro_rules! log {
    ($str_format: expr) => {
        #[cfg(debug_assertions)]
        {
            // const f: &str = file!();
            // const ln: u32 = line!();
            // let fd = $crate::make_fd(f, ln);

            print!("[ci] {}", std::format_args!($str_format));
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            // const f: &str = std::file!();
            // const ln: u32 = std::line!();
            // let fd = $crate::make_fd(f, ln);

            print!("[ci] {}", format_args!($str_format, $($arg)*));
        }
    };
}
