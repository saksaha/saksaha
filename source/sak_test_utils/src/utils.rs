use log::info;
use sak_kv_db::{Options, DB};

use crate::TestUtilsError;

const APP_NAME: &str = "saksaha";

pub fn init_test_log() {
    const RUST_LOG_ENV: &str = "
            sak_,
            saksaha
        ";

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    sak_logger::init(false).unwrap();
}
