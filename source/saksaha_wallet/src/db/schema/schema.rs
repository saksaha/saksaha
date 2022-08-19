use crate::db::{raw::Raw, schema::cfs};
use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;

pub(crate) struct WalletDBSchema {
    pub raw: Raw,
}

impl WalletDBSchema {
    pub(crate) fn new(db: DB) -> WalletDBSchema {
        let raw = Raw { db };

        WalletDBSchema { raw }
    }

    pub(crate) fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(cfs::RHO, Options::default()),
            ColumnFamilyDescriptor::new(cfs::R, Options::default()),
            ColumnFamilyDescriptor::new(cfs::S, Options::default()),
            ColumnFamilyDescriptor::new(cfs::V, Options::default()),
            ColumnFamilyDescriptor::new(cfs::A_SK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::A_PK, Options::default()),
            ColumnFamilyDescriptor::new(cfs::COIN_STATUS, Options::default()),
            ColumnFamilyDescriptor::new(cfs::USER_ID, Options::default()),
            ColumnFamilyDescriptor::new(cfs::CM, Options::default()),
            ColumnFamilyDescriptor::new(cfs::COIN_IDX, Options::default()),
            ColumnFamilyDescriptor::new(cfs::TX_HASH, Options::default()),
            ColumnFamilyDescriptor::new(
                cfs::TX_HASH_STATUS,
                Options::default(),
            ),
        ]
    }
}
