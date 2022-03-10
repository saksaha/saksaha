macro_rules! info {
    ($str_format: expr) => {
        // #[cfg(debug_assertions)]
        {
            logger::tinfo!("sak", $str_format);
            // log::info!("[{}] {}", $tag, $str_format);
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        {
            logger::tinfo!("sak", $str_format, $($arg)*);
        };
    }
}

pub(crate) use info;
