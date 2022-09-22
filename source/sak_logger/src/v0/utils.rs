use crate::{LoggerError, RUST_LOG_ENV};

pub(crate) fn set_rust_log_env() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    let rust_log_env = std::env::var("RUST_LOG");
    println!("sak_logger: RUST_LOG is set to {:?}", rust_log_env);
}
