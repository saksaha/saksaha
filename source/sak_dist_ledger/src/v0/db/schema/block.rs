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
    pub(crate) fn get_validator_sig(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::VALIDATOR_SIG)?;

        match db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_tx_hashes(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASHES)?;

        match db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                return Ok(Some(th));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_witness_sigs(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::WITNESS_SIGS)?;

        match db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                return Ok(Some(th));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_created_at(
        &self,
        db: &DB,
        key: &BlockHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_CREATED_AT)?;

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

    pub(crate) fn get_block_height(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_HEIGHT)?;

        match db.get_cf(&cf, block_hash)? {
            Some(h) => {
                let height = sak_kv_db::convert_u8_slice_into_u128(&h)?;

                return Ok(Some(height));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_hash(
        &self,
        db: &DB,
        block_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_HASH)?;

        let v = block_height.to_be_bytes();

        match db.get_cf(&cf, v)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_cm_count(
        &self,
        db: &DB,
        key: &BlockHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_CM_COUNT)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let val = sak_kv_db::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_merkle_rt(
        &self,
        db: &DB,
        key: &BlockHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_MERKLE_RT)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr: [u8; 32] = match v.try_into() {
                    Ok(a) => a,
                    Err(_) => {
                        return Err(
                            format!("Cannot convert cm into an array",).into()
                        )
                    }
                };

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_prf_merkle_rt(
        &self,
        db: &DB,
        key: &BlockHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::PRF_MERKLE_RT)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr: [u8; 32] = match v.try_into() {
                    Ok(a) => a,
                    Err(err) => {
                        return Err(
                            format!("Cannot convert cm into an array",).into()
                        )
                    }
                };

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
    pub(crate) fn batch_put_validator_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        validator_sig: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::VALIDATOR_SIG)?;

        batch.put_cf(&cf, block_hash, validator_sig);

        Ok(())
    }

    pub(crate) fn batch_put_witness_sigs(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        witness_sigs: &Vec<String>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::WITNESS_SIGS)?;

        let witness_sigs = serde_json::to_string(witness_sigs)?;

        batch.put_cf(&cf, block_hash, witness_sigs);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hashes(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        tx_hashes: &Vec<String>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASHES)?;

        let transactions = serde_json::to_string(tx_hashes)?;

        batch.put_cf(&cf, block_hash, transactions);

        Ok(())
    }

    pub(crate) fn batch_put_block_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_put_block_cm_count(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        cm_count: u128,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_CREATED_AT)?;

        let v = cm_count.to_be_bytes();

        batch.put_cf(&cf, block_hash, &v);

        Ok(())
    }

    pub(crate) fn batch_put_block_hash(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_height: &u128,
        block_hash: &BlockHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_HASH)?;

        let v = block_height.to_be_bytes();

        batch.put_cf(&cf, &v, block_hash);

        Ok(())
    }

    pub(crate) fn batch_put_block_height(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        block_height: &u128,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_HEIGHT)?;

        let v = block_height.to_be_bytes();

        batch.put_cf(&cf, block_hash, v);

        Ok(())
    }

    pub(crate) fn batch_put_block_merkle_rt(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &BlockHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(db, cfs::BLOCK_MERKLE_RT)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
