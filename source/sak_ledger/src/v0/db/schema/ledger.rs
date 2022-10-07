use crate::{cfs, keys, LedgerDB};
use crate::{MachineError, MerkleNodeLoc};
use sak_crypto::ScalarExt;
use sak_kv_db::DB;
use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options, WriteBatch};
use sak_types::{BlockHash, CtrAddr, TxHash, TxType};
use std::convert::TryInto;
use std::sync::Arc;
use type_extension::U8Array;

impl LedgerDB {
    pub(crate) fn get_merkle_node(
        &self,
        // db: &DB,
        key: &String,
    ) -> Result<[u8; 32], MachineError> {
        let cf = self.make_cf_handle(&self.db, cfs::MERKLE_NODE)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(arr);
            }
            None => {
                let zero_value = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };
                return Ok(zero_value.to_bytes());
            }
        }
    }

    // pub(crate) fn get_cm_by_cm_idx(
    //     &self,
    //     cm_idx: &u128,
    // ) -> Result<Option<[u8; 32]>, MachineError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM)?;

    //     let key = cm_idx.to_be_bytes();

    //     match self.db.get_cf(&cf, key)? {
    //         Some(v) => {
    //             let arr = type_extension::convert_vec_into_u8_32(v)?;
    //             return Ok(Some(arr));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    // pub(crate) fn get_ledger_cm_count(
    //     &self,
    //     // db: &DB,
    // ) -> Result<Option<u128>, MachineError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::LEDGER_CM_COUNT)?;

    //     match self.db.get_cf(&cf, keys::SINGLETON)? {
    //         Some(v) => {
    //             let val = type_extension::convert_u8_slice_into_u128(&v)?;

    //             return Ok(Some(val));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    pub(crate) fn get_latest_block_height(
        &self,
        // db: &DB,
    ) -> Result<Option<u128>, MachineError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_HASH)?;

        let mut iter = self.db.iterator_cf(&cf, IteratorMode::End);

        let (height_bytes, _hash) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let height = type_extension::convert_u8_slice_into_u128(&height_bytes)?;

        Ok(Some(height))
    }

    // pub(crate) fn get_latest_tx_height(
    //     &self,
    //     // db: &DB,
    // ) -> Result<Option<u128>, MachineError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_HEIGHT)?;

    //     let mut iter = self.db.iterator_cf(&cf, IteratorMode::End);

    //     let (height_bytes, _hash) = match iter.next() {
    //         Some(a) => a,
    //         None => return Ok(None),
    //     };

    //     let height = type_extension::convert_u8_slice_into_u128(&height_bytes)?;

    //     Ok(Some(height))
    // }
}
