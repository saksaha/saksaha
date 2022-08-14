use crate::{LedgerDBSchema, LedgerError};
use log::info;
use sak_kv_db::{KeyValueDatabase, Options};
use std::path::PathBuf;

// TODO This has to be dynamically decided
const APP_NAME: &'static str = "saksaha";

pub struct LedgerDB {
    pub(crate) schema: LedgerDBSchema,
}

impl LedgerDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<LedgerDB, LedgerError> {
        let ledger_db_path = {
            let db_path = Self::get_db_path(app_prefix)?;

            if !db_path.exists() {
                std::fs::create_dir_all(db_path.clone())?;
            }

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(
            ledger_db_path,
            options,
            LedgerDBSchema::make_cf_descriptors(),
        ) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing key value database, err: {}",
                    err
                )
                .into());
            }
        };

        let schema = LedgerDBSchema::new(kv_db.db_instance);

        let database = LedgerDB { schema };

        info!("Initialized Database");

        Ok(database)
    }

    pub fn get_db_path(app_prefix: &str) -> Result<PathBuf, LedgerError> {
        let app_path = sak_fs::get_app_root_path(APP_NAME)?.join(app_prefix);

        let db_path = app_path.join("db").join("ledger");

        Ok(db_path)
    }
}
