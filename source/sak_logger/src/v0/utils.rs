use crate::{LoggerError, RUST_LOG_ENV};

pub(crate) fn set_rust_log_env() -> String {
    match std::env::var("RUST_LOG") {
        Ok(l) => l,
        Err(_) => {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
            RUST_LOG_ENV.to_string()
        }
    }
}
