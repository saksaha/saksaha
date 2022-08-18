use crate::TestUtilsError;
use log::info;
use sak_kv_db::{Options, DB};
use sak_logger::RUST_LOG_ENV;

const APP_NAME: &str = "saksaha";

pub fn init_test_log() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    let _ = sak_logger::init(false);
}
