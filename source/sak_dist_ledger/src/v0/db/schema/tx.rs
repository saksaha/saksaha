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

// getter
impl LedgerDBSchema {
    pub(crate) fn get_tx_type(
        &self,
        db: &DB,
        tx_hash: &TxHash,
    ) -> Result<Option<TxType>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_TYPE)?;

        match db.get_cf(&cf, tx_hash)? {
            Some(v) => {
                let tx_type = match v.get(0) {
                    Some(t) => TxType::from(*t),
                    None => {
                        return Err(format!("tx type is corrupted").into());
                    }
                };

                return Ok(Some(tx_type));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_tx_hash_by_height(
        &self,
        db: &DB,
        tx_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASH)?;

        let key = tx_height.to_be_bytes();

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

    pub(crate) fn get_tx_created_at(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_CREATED_AT)?;

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

    pub(crate) fn get_data(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::DATA)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_author_sig(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::AUTHOR_SIG)?;

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

    pub(crate) fn get_pi(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::PI)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_ctr_addr(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CTR_ADDR)?;

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

    pub(crate) fn get_tx_height(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HEIGHT)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let height = sak_kv_db::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(height));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_v(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::V)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_k(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::K)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_s(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::S)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_sn_1(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::SN_1)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_sn_2(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::SN_2)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_1(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM_1)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_2(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM_2)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = sak_kv_db::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }
}

// writer
impl LedgerDBSchema {
    pub(crate) fn batch_put_tx_type(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        tx_hash: &TxHash,
        tx_type: TxType,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_TYPE)?;

        println!("put tx type, hash: {:?}", tx_hash);

        batch.put_cf(&cf, tx_hash, &[tx_type as u8]);

        Ok(())
    }

    pub(crate) fn batch_put_tx_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &TxHash,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_delete_tx_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_CREATED_AT)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_data(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::DATA)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_data(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::DATA)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_pi(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::PI)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_pi(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::PI)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_author_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::AUTHOR_SIG)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_author_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::AUTHOR_SIG)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_ctr_addr(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CTR_ADDR)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_tx_height(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        tx_height: &u128,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HEIGHT)?;

        let v = tx_height.to_be_bytes();

        batch.put_cf(&cf, block_hash, v);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hash_by_height(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        tx_height: &u128,
        tx_hash: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASH)?;

        let v = tx_height.to_be_bytes();

        batch.put_cf(&cf, v, tx_hash);

        Ok(())
    }

    pub(crate) fn batch_put_cm(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_v(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::V)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_k(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::K)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_s(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::S)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_1(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::SN_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_2(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::SN_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_1(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_2(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::CM_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_prf_merkle_rt(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::PRF_MERKLE_RT)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
