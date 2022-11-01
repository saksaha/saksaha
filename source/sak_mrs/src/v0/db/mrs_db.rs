use super::columns::{self, cfs, CFSenum};

use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, DBIteratorWithThreadMode, DBWithThreadMode,
    IteratorMode, KeyValueDatabase, MultiThreaded, Options, WriteBatch, DB,
};

use crate::MRSError;
use sak_logger::info;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct MRSDB {
    pub(crate) db: DB,
}
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MrsEntity {
    pub mrs_key: String,
    pub mrs_value: String,
    pub ib: Vec<u8>,
    pub timestamp: String,
    pub idx: u32,
}

impl MRSDB {
    pub(crate) async fn init<P: AsRef<Path>>(db_path: P) -> Result<MRSDB, MRSError> {
        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(db_path, options, Self::make_cf_descriptors()) {
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
        vec![
            ColumnFamilyDescriptor::new(CFSenum::MrsKey.as_str(), Options::default()),
            ColumnFamilyDescriptor::new(CFSenum::MrsValue.as_str(), Options::default()),
            ColumnFamilyDescriptor::new(CFSenum::IntegrityBits.as_str(), Options::default()),
            ColumnFamilyDescriptor::new(CFSenum::Timestamp.as_str(), Options::default()),
            ColumnFamilyDescriptor::new(CFSenum::Idx.as_str(), Options::default()),
            // ColumnFamilyDescriptor::new(cfs::MRS_KEY, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::MRS_VALUE, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::INTEGRITY_BITS, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::TIMESTAMP, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::IDX, Options::default()),
        ]
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

    pub fn put_ser<T: Serialize>(
        &self,
        batch: &mut WriteBatch,
        column: CFSenum,
        key: &[u8],
        value: &T,
    ) -> Result<(), MRSError> {
        let data = serde_json::to_vec(value)?;

        self.put(batch, column, key, &data)?;

        Ok(())
    }

    pub fn put(
        &self,
        batch: &mut WriteBatch,
        column: CFSenum,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), MRSError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub fn get_ser<T: Serialize + DeserializeOwned>(
        &self,
        column: CFSenum,
        key: &[u8],
    ) -> Result<Option<T>, MRSError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        match self.db.get_cf(&cf, key)? {
            Some(ref v) => {
                let arr = serde_json::from_slice(v)?;

                Ok(Some(arr))
            }
            None => Ok(None),
        }
    }

    pub fn iter(
        &self,
        column: CFSenum,
    ) -> Result<DBIteratorWithThreadMode<DBWithThreadMode<MultiThreaded>>, MRSError> {
        let cf = self.make_cf_handle(&self.db, column.as_str())?;

        Ok(self.db.iterator_cf(&cf, IteratorMode::End))
    }
}
