use sak_kv_db::{Options, DB};
use sak_logger::{info, SakLogger};
use std::{fs, path::Path};

const APP_NAME: &str = "saksaha";

pub struct SakMachineTestUtils;

impl SakMachineTestUtils {
    pub fn init_saksaha_test(pk_str: String) {
        SakLogger::init_test_console().unwrap();
        let test_ledger_path = {
            let s = "/tmp/saksaha_test";
            Path::new(s).join(pk_str).join("ledger")
        };

        if test_ledger_path.is_dir() {
            DB::destroy(&Options::default(), test_ledger_path).unwrap();
        }

        info!("Initialized test configurations");
    }
}
