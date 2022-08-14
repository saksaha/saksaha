use crate::{LedgerDBSchema, LedgerError};
use log::info;
use sak_kv_db::{KeyValueDatabase, Options};

pub(crate) struct LedgerDB {
    pub(crate) schema: LedgerDBSchema,
}

impl LedgerDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<LedgerDB, LedgerError> {
        let ledger_db_path = {
            let app_path =
                sak_fs::create_or_get_app_path("saksaha")?.join(app_prefix);

            let db_path = { app_path.join("db").join("ledger") };

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
}
