use crate::db::schema::cfs;

use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options, WriteBatch, DB};
use std::sync::Arc;

pub(crate) struct EnvelopeDBSchema {
    pub(crate) db: DB,
}

impl EnvelopeDBSchema {
    pub(crate) fn new(db: DB) -> EnvelopeDBSchema {
        EnvelopeDBSchema { db }
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(cfs::MY_SK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::MY_PK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::MY_SIG, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CH_ID, Options::default()),
            ColumnFamilyDescriptor::new(cfs::HER_PK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::AES_KEY, Options::default()),
            ColumnFamilyDescriptor::new(cfs::ACC_ADDR, Options::default()),
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
}
