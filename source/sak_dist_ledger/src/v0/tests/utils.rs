use crate::LedgerDB;
use sak_kv_db::{Options, DB};
use sak_logger::{info, SakLogger};

const APP_NAME: &str = "saksaha";

pub fn init_test(app_prefixes: Vec<&str>) {
    SakLogger::init_test_console().unwrap();

    for app_prefix in app_prefixes {
        let db_path = {
            let config_dir = sak_dir::get_config_dir(APP_NAME).unwrap();
            let p = config_dir.join(app_prefix).join("db/ledger");
            p
        };

        if db_path.is_dir() {
            DB::destroy(&Options::default(), db_path).unwrap();
        }
    }

    info!("Initialized test configurations");
}
// }
