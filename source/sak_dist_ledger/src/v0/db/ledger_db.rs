use crate::{LedgerDBSchema, LedgerError};
use log::info;
use sak_fs::FS;
use sak_kv_db::{KeyValueDatabase, Options};

pub(crate) struct LedgerDB {
    pub(crate) kv_db: KeyValueDatabase,
    pub(super) schema: LedgerDBSchema,
}

impl LedgerDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<LedgerDB, LedgerError> {
        let ledger_db_path = {
            let app_path = FS::create_or_get_app_path(app_prefix)?;
            let db_path = { app_path.join("db").join("ledger") };

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let schema = LedgerDBSchema::new();

        let kv_db = match KeyValueDatabase::new(
            ledger_db_path,
            options,
            schema.make_cf_descriptors(),
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

        let database = LedgerDB { kv_db, schema };

        info!("Initialized Database");

        Ok(database)
    }
}
