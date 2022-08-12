mod io;

use std::sync::Arc;

use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;
use sak_crypto::{Scalar, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};
use sak_proofs::{OldCoin, CM_TREE_DEPTH};
use sak_types::CoinStatus;
use type_extension::U8Arr32;

pub(crate) struct Raw {
    db: DB,
}

impl Raw {
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
