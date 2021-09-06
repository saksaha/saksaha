#[macro_export]
macro_rules! log {
    (DEBUG $str_format: expr) => {
        let f = std::file!();
        let ln = std::line!();

        println!("{}:{} {}", f, ln, format_args!($str_format));
    };

    (DEBUG $str_format: expr, $($arg:tt)*) => {
        let f = std::file!();
        let ln = std::line!();

        println!("{}:{} {}", f, ln, format_args!($str_format, $($arg)*));
    };
}
