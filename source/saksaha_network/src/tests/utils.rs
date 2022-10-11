use crate::fs::{self, SaksahaFS};
use sak_kv_db::{Options, DB};
use sak_logger::{info, SakLogger};
use std::path::PathBuf;

pub struct SaksahaTestUtils {}

impl SaksahaTestUtils {
    pub fn init_test(public_keys: &[&str]) {
        let log_root_dir = SaksahaFS::config_dir().unwrap();

        SakLogger::init_test_console().unwrap();

        for pk in public_keys {
            let ledger_path = get_ledger_path(&pk.to_string());

            if ledger_path.exists() {
                DB::destroy(&Options::default(), ledger_path).expect("Cannot open a file for lock");
            }
        }

        info!("Initialized test configurations");
    }
}

fn get_ledger_path(pk: &String) -> PathBuf {
    SaksahaFS::acc_dir(pk).unwrap().join("db/ledger")
}
