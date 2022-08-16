use crate::cfs;
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options,
    WriteBatch, DB,
};
use std::sync::Arc;

pub(crate) struct LedgerDBSchema {
    pub(super) db: DB,
}

impl LedgerDBSchema {
    pub(crate) fn new(db: DB) -> LedgerDBSchema {
        LedgerDBSchema { db }
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(
                cfs::TX_HASH_BY_HEIGHT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(
                cfs::TX_HASH_BY_CTR_ADDR,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::TX_HASH_BY_SN, Options::default()),
            ColumnFamilyDescriptor::new(cfs::PI, Options::default()),
            ColumnFamilyDescriptor::new(cfs::AUTHOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_CREATED_AT, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::BLOCK_CREATED_AT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::DATA, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CTR_ADDR, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_TYPE, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM, Options::default()),
            ColumnFamilyDescriptor::new(cfs::V, Options::default()),
            ColumnFamilyDescriptor::new(cfs::K, Options::default()),
            ColumnFamilyDescriptor::new(cfs::S, Options::default()),
            ColumnFamilyDescriptor::new(cfs::SN_1, Options::default()),
            ColumnFamilyDescriptor::new(cfs::SN_2, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_1, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM_2, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::BLOCK_MERKLE_RT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(cfs::PRF_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::MERKLE_NODE, Options::default()),
            ColumnFamilyDescriptor::new(cfs::VALIDATOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_HASHES, Options::default()),
            ColumnFamilyDescriptor::new(cfs::WITNESS_SIGS, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(cfs::BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CTR_STATE, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::BLOCK_CM_COUNT,
                Options::default(),
            ),
            ColumnFamilyDescriptor::new(
                cfs::LEDGER_CM_COUNT,
                Options::default(),
            ),
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
