mod columns;

use std::path::PathBuf;

use database::KeyValueDatabase;
use file_system::FS;
use rocksdb::{ColumnFamilyDescriptor, Options};

const LEDGER_DB_PATH: &str = "db_ledger";

pub(crate) struct DB {
    pub(crate) ledger_db: KeyValueDatabase,
}

impl DB {
    pub(crate) async fn init() -> Result<DB, String> {
        let ledger_db = DB::init_ledger_db()?;

        let db = DB { ledger_db };

        Ok(db)
    }

    fn init_ledger_db() -> Result<KeyValueDatabase, String> {
        let db_path = {
            let app_path = FS::get_app_path()?;

            app_path.join(LEDGER_DB_PATH)
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let cf_descriptors = columns::make_ledger_cf_descriptors();

        let ledger_db =
            match KeyValueDatabase::new(db_path, options, cf_descriptors) {
                Ok(d) => d,
                Err(err) => {
                    return Err(format!(
                        "Error initializing key value database, err: {}",
                        err
                    ));
                }
            };

        Ok(ledger_db)
    }
}

fn make_db_path() -> Result<PathBuf, String> {
    let app_path = FS::get_app_path()?;
    let db_path = app_path.join(LEDGER_DB_PATH);

    Ok(db_path)
}

mod dummy {
    pub(crate) struct Transaction<'a> {
        tx_hash: &'a str,
        pi: &'a str,
        contract_addr: &'a str,
        data: &'a str,
        fee: f32,
    }

    impl<'a> Transaction<'a> {
        pub(crate) fn new(
            tx_hash: &'a str,
            pi: &'a str,
            contract_addr: &'a str,
            data: &'a str,
            fee: f32,
        ) -> Transaction<'a> {
            Transaction {
                tx_hash,
                pi,
                contract_addr,
                data,
                fee,
            }
        }
    }
}
