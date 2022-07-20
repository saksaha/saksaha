use crate::{LedgerError, MerkleNodeLoc};
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options,
    WriteBatch, DB,
};
use sak_types::{BlockHash, CtrAddr, TxHash, TxType};
use std::convert::TryInto;
use std::sync::Arc;

mod keys {
    pub(super) const SINGLETON: &[u8; 1] = &[0];
}

const TX_HASH: &str = "tx_hash";

const TX_TYPE: &str = "tx_type";

const PI: &str = "pi";

const AUTHOR_SIG: &str = "author_sig";

const TX_CREATED_AT: &str = "tx_created_at";

const BLOCK_CREATED_AT: &str = "block_created_at";

const DATA: &str = "data";

const CTR_ADDR: &str = "ctr_addr";

const TX_HEIGHT: &str = "tx_height";

const CM: &str = "cm";

const V: &str = "v";

const K: &str = "k";

const S: &str = "s";

const SN_1: &str = "sn_1";

const SN_2: &str = "sn_2";

const CM_1: &str = "cm_1";

const CM_2: &str = "cm_2";

const BLOCK_CM_COUNT: &str = "block_cm_count";

const LEDGER_CM_COUNT: &str = "ledger_cm_count";

const BLOCK_MERKLE_RT: &str = "block_merkle_rt";

const PRF_MERKLE_RT: &str = "prf_merkle_rt";

const MERKLE_NODE: &str = "merkle_node";

const VALIDATOR_SIG: &str = "validator_sig";

const TX_HASHES: &str = "tx_hashes";

const WITNESS_SIGS: &str = "witness_sigs";

const BLOCK_HEIGHT: &str = "block_height";

const BLOCK_HASH: &str = "block_hash";

const CTR_STATE: &str = "ctr_state";

pub(crate) struct LedgerDBSchema {}

impl LedgerDBSchema {
    pub(crate) fn new() -> LedgerDBSchema {
        LedgerDBSchema {}
    }

    pub(crate) fn get_validator_sig(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, VALIDATOR_SIG)?;

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

    pub(crate) fn get_tx_type(
        &self,
        db: &DB,
        tx_hash: &TxHash,
    ) -> Result<Option<TxType>, LedgerError> {
        let cf = make_cf_handle(db, TX_TYPE)?;

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

    pub(crate) fn get_tx_hashes(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = make_cf_handle(db, TX_HASHES)?;

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

    pub(crate) fn get_tx_hash_by_height(
        &self,
        db: &DB,
        tx_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, TX_HASH)?;

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

    pub(crate) fn get_witness_sigs(
        &self,
        db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = make_cf_handle(db, WITNESS_SIGS)?;

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

    pub(crate) fn get_tx_created_at(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, TX_CREATED_AT)?;

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

    pub(crate) fn get_block_created_at(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, BLOCK_CREATED_AT)?;

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
        let cf = make_cf_handle(db, BLOCK_HEIGHT)?;

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
        let cf = make_cf_handle(db, BLOCK_HASH)?;

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

    pub(crate) fn get_data(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, DATA)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_merkle_node(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = make_cf_handle(db, MERKLE_NODE)?;

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

    pub(crate) fn get_tx_hash(
        &self,
        db: &DB,
        key: &CtrAddr,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, TX_HASH)?;

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
        let cf = make_cf_handle(db, CTR_STATE)?;

        match db.get_cf(&cf, ctr_addr)? {
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
        let cf = make_cf_handle(db, AUTHOR_SIG)?;

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

    pub(crate) fn batch_put_validator_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        validator_sig: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, VALIDATOR_SIG)?;

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
        let cf = make_cf_handle(db, WITNESS_SIGS)?;

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
        let cf = make_cf_handle(db, TX_HASHES)?;

        let transactions = serde_json::to_string(tx_hashes)?;

        batch.put_cf(&cf, block_hash, transactions);

        Ok(())
    }

    pub(crate) fn batch_put_tx_type(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        tx_hash: &TxHash,
        tx_type: TxType,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, TX_TYPE)?;

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
        let cf = make_cf_handle(db, TX_CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_put_block_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, BLOCK_CREATED_AT)?;

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
        let cf = make_cf_handle(db, BLOCK_CREATED_AT)?;

        let v = cm_count.to_be_bytes();

        batch.put_cf(&cf, block_hash, &v);

        Ok(())
    }

    pub(crate) fn batch_put_ledger_cm_count(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        cm_count: u128,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, LEDGER_CM_COUNT)?;

        let v = cm_count.to_be_bytes();

        batch.put_cf(&cf, keys::SINGLETON, &v);

        Ok(())
    }

    pub(crate) fn batch_put_merkle_node(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        merkle_node_loc: &MerkleNodeLoc,
        node_val: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, MERKLE_NODE)?;

        batch.put_cf(&cf, merkle_node_loc, node_val);

        Ok(())
    }

    pub(crate) fn batch_delete_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, TX_CREATED_AT)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_block_hash(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_height: &u128,
        block_hash: &BlockHash,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, BLOCK_HASH)?;

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
        let cf = make_cf_handle(db, BLOCK_HEIGHT)?;

        let v = block_height.to_be_bytes();

        batch.put_cf(&cf, block_hash, v);

        Ok(())
    }

    pub(crate) fn batch_put_ctr_state(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        ctr_addr: &CtrAddr,
        ctr_state: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CTR_STATE)?;

        batch.put_cf(&cf, ctr_addr, ctr_state);

        Ok(())
    }

    pub(crate) fn batch_put_data(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, DATA)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_data(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, DATA)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn get_pi(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, PI)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn batch_put_pi(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, PI)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_pi(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, PI)?;

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
        let cf = make_cf_handle(db, AUTHOR_SIG)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_author_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, AUTHOR_SIG)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn get_ctr_addr(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, CTR_ADDR)?;

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
        let cf = make_cf_handle(db, TX_HEIGHT)?;

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
        let cf = make_cf_handle(db, CM)?;

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

    pub(crate) fn get_cm_by_height(
        &self,
        db: &DB,
        tx_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, CM)?;

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

    pub(crate) fn get_v(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = make_cf_handle(db, V)?;

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
        let cf = make_cf_handle(db, K)?;

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
        let cf = make_cf_handle(db, S)?;

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
        let cf = make_cf_handle(db, SN_1)?;

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
        let cf = make_cf_handle(db, SN_2)?;

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
        let cf = make_cf_handle(db, CM_1)?;

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
        let cf = make_cf_handle(db, CM_2)?;

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

    pub(crate) fn get_ledger_cm_count(
        &self,
        db: &DB,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = make_cf_handle(db, LEDGER_CM_COUNT)?;

        match db.get_cf(&cf, keys::SINGLETON)? {
            Some(v) => {
                let val = sak_kv_db::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(val));
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
        let cf = make_cf_handle(db, BLOCK_CM_COUNT)?;

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
        let cf = make_cf_handle(db, BLOCK_MERKLE_RT)?;

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

    pub(crate) fn get_prf_merkle_rt(
        &self,
        db: &DB,
        key: &BlockHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = make_cf_handle(db, PRF_MERKLE_RT)?;

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

    pub(crate) fn batch_put_ctr_addr(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CTR_ADDR)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hash(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &CtrAddr,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, TX_HASH)?;

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
        let cf = make_cf_handle(db, TX_HEIGHT)?;

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
        let cf = make_cf_handle(db, TX_HASH)?;

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
        let cf = make_cf_handle(db, CM)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_by_height(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        tx_height: &u128,
        cm: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CM)?;

        let v = tx_height.to_be_bytes();

        batch.put_cf(&cf, v, cm);

        Ok(())
    }

    pub(crate) fn batch_put_v(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, V)?;

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
        let cf = make_cf_handle(db, K)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_s(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, S)?;

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
        let cf = make_cf_handle(db, SN_1)?;

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
        let cf = make_cf_handle(db, SN_2)?;

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
        let cf = make_cf_handle(db, CM_1)?;

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
        let cf = make_cf_handle(db, CM_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_merkle_rt(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &BlockHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, BLOCK_MERKLE_RT)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn get_latest_block_height(
        &self,
        db: &DB,
    ) -> Result<Option<u128>, String> {
        let cf = make_cf_handle(db, BLOCK_HASH)?;

        let mut iter = db.iterator_cf(&cf, IteratorMode::End);

        let (height_bytes, _hash) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let height = sak_kv_db::convert_u8_slice_into_u128(&height_bytes)?;

        Ok(Some(height))
    }

    // pub(crate) fn get_latest_cm_idx(
    //     &self,
    //     db: &DB,
    // ) -> Result<Option<u128>, String> {
    //     let cf = make_cf_handle(db, BLOCK_HASH)?;

    //     let mut iter = db.iterator_cf(&cf, IteratorMode::End);

    //     let (height_bytes, _hash) = match iter.next() {
    //         Some(a) => a,
    //         None => return Ok(None),
    //     };

    //     let height = sak_kv_db::convert_u8_slice_into_u128(&height_bytes)?;

    //     Ok(Some(height))
    // }

    pub(crate) fn get_latest_tx_height(
        &self,
        db: &DB,
    ) -> Result<Option<u128>, String> {
        let cf = make_cf_handle(db, TX_HEIGHT)?;

        let mut iter = db.iterator_cf(&cf, IteratorMode::End);

        let (_hash, height_bytes) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let height = sak_kv_db::convert_u8_slice_into_u128(&height_bytes)?;

        Ok(Some(height))
    }

    pub(crate) fn make_cf_descriptors(&self) -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(TX_HASH, Options::default()),
            ColumnFamilyDescriptor::new(PI, Options::default()),
            ColumnFamilyDescriptor::new(AUTHOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(TX_CREATED_AT, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_CREATED_AT, Options::default()),
            ColumnFamilyDescriptor::new(DATA, Options::default()),
            ColumnFamilyDescriptor::new(CTR_ADDR, Options::default()),
            ColumnFamilyDescriptor::new(TX_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(TX_TYPE, Options::default()),
            ColumnFamilyDescriptor::new(CM, Options::default()),
            ColumnFamilyDescriptor::new(V, Options::default()),
            ColumnFamilyDescriptor::new(K, Options::default()),
            ColumnFamilyDescriptor::new(S, Options::default()),
            ColumnFamilyDescriptor::new(SN_1, Options::default()),
            ColumnFamilyDescriptor::new(SN_2, Options::default()),
            ColumnFamilyDescriptor::new(CM_1, Options::default()),
            ColumnFamilyDescriptor::new(CM_2, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(PRF_MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(MERKLE_NODE, Options::default()),
            ColumnFamilyDescriptor::new(VALIDATOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(TX_HASHES, Options::default()),
            ColumnFamilyDescriptor::new(WITNESS_SIGS, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(CTR_STATE, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_CM_COUNT, Options::default()),
            ColumnFamilyDescriptor::new(LEDGER_CM_COUNT, Options::default()),
        ]
    }
}

fn make_cf_handle<'a>(
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
