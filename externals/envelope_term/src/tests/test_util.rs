use crate::{db::EnvelopeDB, fs};
use log::info;
use sak_kv_db::{Options, DB};
use std::path::Path;

pub(crate) struct TestUtil;

impl TestUtil {
    pub fn init_test(app_prefixes: Vec<&str>) {
        for app_prefix in app_prefixes {
            let acc_dir = fs::acc_dir(&app_prefix.to_string()).unwrap();

            let db_path = EnvelopeDB::get_db_path(&acc_dir).unwrap();

            if Path::new(&db_path).exists() {
                DB::destroy(&Options::default(), db_path).expect("Cannot open a file for lock");
            }
        }

        info!("Initialized test configurations");
    }
}
