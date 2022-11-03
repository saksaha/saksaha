use crate::{SakMRS, SakMRSArgs};
use sak_credential::{Credential as SakCredential, CredentialProfile};
use sak_kv_db::{Options, DB};
use sak_logger::{info, SakLogger};
use std::{fs, path::Path};

pub(crate) struct MRSTestUtils;

impl MRSTestUtils {
    pub async fn mock_mrs_db() -> SakMRS {
        SakLogger::init_test_console().unwrap();
        let credential = CredentialProfile::test_1();

        Self::init_saksaha_test(credential.public_key_str.clone());

        let test_dir = {
            let tempdir = std::env::temp_dir()
                .join("saksaha_test")
                .join(credential.public_key_str);

            std::fs::create_dir_all(&tempdir).unwrap();
            tempdir
        };

        let mrs_path = { test_dir.join("mrs") };

        let mrs = {
            let mrs_args = SakMRSArgs {
                mrs_db_path: mrs_path,
            };

            let m = SakMRS::init(mrs_args).await.unwrap();
            m
        };

        mrs
    }

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
