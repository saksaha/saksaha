use crate::fs;
use log::info;
use sak_dist_ledger::LedgerDB;
use sak_kv_db::{Options, DB};
use std::path::{Path, PathBuf};

pub(crate) struct TestUtil;

impl TestUtil {
    pub fn init_test(public_keys: Vec<&str>) {
        for pk in public_keys {
            // let db_path = LedgerDB::get_db_path(pk).unwrap();
            let ledger_path = get_ledger_path(&pk.to_string());

            if ledger_path.exists() {
                DB::destroy(&Options::default(), ledger_path)
                    .expect("Cannot open a file for lock");
            }
        }

        info!("Initialized test configurations");
    }
}

fn get_ledger_path(pk: &String) -> PathBuf {
    fs::acc_dir(pk).unwrap().join("db/ledger")
}
