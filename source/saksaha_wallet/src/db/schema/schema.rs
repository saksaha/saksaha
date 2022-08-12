use crate::db::schema::cfs;
use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;

pub(crate) struct WalletDBSchema {
    pub(crate) db: DB,
}

impl WalletDBSchema {
    pub(crate) fn new(db: DB) -> WalletDBSchema {
        WalletDBSchema { db }
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(cfs::RHO, Options::default()),
            ColumnFamilyDescriptor::new(cfs::R, Options::default()),
            ColumnFamilyDescriptor::new(cfs::S, Options::default()),
            ColumnFamilyDescriptor::new(cfs::V, Options::default()),
            ColumnFamilyDescriptor::new(cfs::A_SK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::A_PK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::STATUS, Options::default()),
            ColumnFamilyDescriptor::new(cfs::USER_ID, Options::default()),
            // ColumnFamilyDescriptor::new(cfs::CM_IDX, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM, Options::default()),
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
                return Err(
                    format!("Fail to open ledger colums {}", col_name,),
                );
            }
        };

        Ok(cf_handle)
    }
}
