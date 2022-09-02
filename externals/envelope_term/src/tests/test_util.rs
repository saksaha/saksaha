use crate::db::EnvelopeDB;
use log::info;
use sak_kv_db::{Options, DB};
use std::path::Path;

pub(crate) struct TestUtil;

impl TestUtil {
    pub fn init_test(app_prefixes: Vec<&str>) {
        for app_prefix in app_prefixes {
            let db_path = EnvelopeDB::get_db_path(app_prefix).unwrap();

            if Path::new(&db_path).exists() {
                DB::destroy(&Options::default(), db_path)
                    .expect("Cannot open a file for lock");
            }
        }

        info!("Initialized test configurations");
    }
}
