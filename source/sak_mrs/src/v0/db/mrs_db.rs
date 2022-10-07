use super::columns::{self, Columns};
use crate::MRSError;
use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, KeyValueDatabase, Options, DB};
use sak_logger::info;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// const APP_NAME: &'static str = "saksaha";

pub struct MRSDB {
    pub(crate) db: DB,
}

impl MRSDB {
    pub(crate) async fn init<P: AsRef<Path>>(db_path: P) -> Result<MRSDB, MRSError> {
        let mrs_db_path = {
            if !db_path.as_ref().clone().exists() {
                std::fs::create_dir_all(db_path.as_ref().clone())?;
            }

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(mrs_db_path, options, Self::make_cf_descriptors()) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("Error initializing key value database, err: {}", err).into());
            }
        };

        let database = MRSDB {
            db: kv_db.db_instance,
        };

        Ok(database)
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![ColumnFamilyDescriptor::new(
            Columns::TX_HASH_BY_CTR_ADDR,
            Options::default(),
        )]
    }

    pub(crate) fn make_cf_handle<'a>(
        &self,
        db: &'a DB,
        col_name: &'static str,
    ) -> Result<Arc<BoundColumnFamily<'a>>, String> {
        let cf_handle = match db.cf_handle(col_name) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger colums {}", col_name,));
            }
        };

        Ok(cf_handle)
    }
}
