use log::info;
use sak_dist_ledger::LedgerDB;
use sak_kv_db::{Options, DB};

pub(crate) struct TestUtil;

impl TestUtil {
    pub fn init_test(app_prefixes: Vec<&str>) {
        for app_prefix in app_prefixes {
            let db_path = LedgerDB::get_db_path(app_prefix).unwrap();

            DB::destroy(&Options::default(), db_path).unwrap();
        }

        info!("Initialized test configurations");
    }
}
