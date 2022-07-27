use log::info;
use sak_kv_db::{Options, DB};

pub fn init_test_log() {
    const RUST_LOG_ENV: &str = "
            sak_,
            saksaha
        ";

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    sak_logger::init(false);
}

pub fn init_test_config(app_prefixes: &Vec<String>) -> Result<(), String> {
    for app_prefix in app_prefixes {
        let db_path = sak_fs::create_or_get_app_path(app_prefix)
            .expect("Failed to create or get the db_path");
        let ledger_path = db_path.join("db").join("ledger");

        if !ledger_path.is_dir() {
            continue;
        }

        let _ = match DB::destroy(&Options::default(), ledger_path.clone()) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };
    }

    info!("Initialized test configurations");

    Ok(())
}
