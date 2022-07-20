use crate::{cfs, LedgerDBSchema};
use crate::{LedgerError, MerkleNodeLoc};
use sak_kv_db::DB;
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options,
    WriteBatch,
};
use sak_types::{BlockHash, CtrAddr, TxHash, TxType};
use std::convert::TryInto;
use std::sync::Arc;

impl LedgerDBSchema {
    pub(crate) fn get_tx_hash(
        &self,
        db: &DB,
        key: &CtrAddr,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASH)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_ctr_state(
        &self,
        db: &DB,
        ctr_addr: &CtrAddr,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CTR_STATE)?;

        match db.get_cf(&cf, ctr_addr)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }
}

// writer
impl LedgerDBSchema {
    pub(crate) fn batch_put_ctr_state(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        ctr_addr: &CtrAddr,
        ctr_state: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CTR_STATE)?;

        batch.put_cf(&cf, ctr_addr, ctr_state);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hash(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &CtrAddr,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASH)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
