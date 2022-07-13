use crate::LedgerError;
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options,
    WriteBatch, DB,
};
use std::convert::TryInto;
use std::sync::Arc;

// const TARGET_BITS: usize = 16;

// CTR_ADDR => Tx Hash
// TX_HEIGHT => TX_HASH
const TX_HASH: &str = "tx_hash";

const TX_TYPE: &str = "tx_type";

// TX_Hash => PI
const PI: &str = "pi";

//TX_Hash  AUTHOR_SIG
const AUTHOR_SIG: &str = "author_sig";

// TX_Hash  CREATED_AT
const CREATED_AT: &str = "created_at";

// TX_Hash       | DATA
const DATA: &str = "data";

//  TX_Hash       | CTR_ADDR
const CTR_ADDR: &str = "ctr_addr";

//  TX_Hash       | TX_HEIGHT
const TX_HEIGHT: &str = "tx_height";

// TX_Hash       | CM
//  TX_HEIGHT     | CM
const CM: &str = "cm";

//  TX_Hash       | V
const V: &str = "v";

//   TX_Hash       | K
const K: &str = "k";

// TX_Hash => S
const S: &str = "s";

//  TX_Hash       | SN_1
const SN_1: &str = "sn_1";

//  TX_Hash       | SN_2
const SN_2: &str = "sn_2";

// TX_Hash => CM_1
const CM_1: &str = "cm_1";

// TX_Hash       | CM_2
const CM_2: &str = "cm_2";

// TX_HASH => MERKLE_RT
const MERKLE_RT: &str = "merkle_rt";

const MERKLE_NODE: &str = "merkle_node";

//// Block
//
const VALIDATOR_SIG: &str = "validator_sig";

//
const TX_HASHES: &str = "tx_hashes";

//
const WITNESS_SIGS: &str = "witness_sigs";

//
const BLOCK_HEIGHT: &str = "block_height";

//
const MERKLE_ROOT: &str = "merkle_root";

//
const BLOCK_HASH: &str = "block_hash";

//
const CTR_STATE: &str = "ctr_state";

pub(crate) struct LedgerDBSchema {}

impl LedgerDBSchema {
    pub(crate) fn new() -> LedgerDBSchema {
        LedgerDBSchema {}
    }

    pub(crate) fn get_validator_sig(
        &self,
        db: &DB,
        block_hash: &String,
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
        tx_hash: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, TX_TYPE)?;

        match db.get_cf(&cf, tx_hash)? {
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
        block_hash: &String,
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
        block_hash: &String,
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

    pub(crate) fn get_created_at(
        &self,
        db: &DB,
        // TODO define enum to include various types of key variants
        key: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, CREATED_AT)?;

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
        block_hash: &String,
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
        key: &String,
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
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, MERKLE_NODE)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_tx_hash(
        &self,
        db: &DB,
        // ctr_addr
        key: &String,
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
        state_key: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, CTR_STATE)?;

        match db.get_cf(&cf, state_key)? {
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
        // tx_hash
        key: &String,
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
        block_hash: &String,
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
        block_hash: &String,
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
        block_hash: &String,
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
        tx_hash: &String,
        tx_type: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, TX_TYPE)?;

        batch.put_cf(&cf, tx_hash, tx_type);

        Ok(())
    }

    pub(crate) fn batch_put_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_hash: &String,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_put_merkle_node(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        location: &String,
        node_val: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, MERKLE_NODE)?;

        batch.put_cf(&cf, location, node_val);

        Ok(())
    }

    pub(crate) fn batch_delete_created_at(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        // tx_hash,
        key: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CREATED_AT)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_block_hash(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        block_height: &u128,
        block_hash: &String,
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
        block_hash: &String,
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
        ctr_addr: &String,
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
        // tx_hash,
        key: &String,
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
        // tx_hash,
        key: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, DATA)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn get_pi(
        &self,
        db: &DB,
        key: &String,
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
        // tx_hash
        key: &String,
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
        // tx_hash
        key: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, PI)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_author_sig(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
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
        // tx_hash
        key: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, AUTHOR_SIG)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn get_ctr_addr(
        &self,
        db: &DB,
        key: &String,
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
        key: &String,
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
        // tx_hash
        key: &String,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = make_cf_handle(db, CM)?;

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
        key: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, V)?;

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

    pub(crate) fn get_k(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, K)?;

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

    pub(crate) fn get_s(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, S)?;

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

    pub(crate) fn get_sn_1(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, SN_1)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_sn_2(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, SN_2)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_1(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, CM_1)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_2(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, CM_2)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_merkle_rt(
        &self,
        db: &DB,
        key: &String,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = make_cf_handle(db, MERKLE_RT)?;

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
        key: &String,
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
        block_hash: &String,
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
        key: &String,
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
        key: &String,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, V)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_k(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &String,
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
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, S)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_1(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, SN_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_2(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, SN_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_1(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CM_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_2(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, CM_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_merkle_rt(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, MERKLE_RT)?;

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
            ColumnFamilyDescriptor::new(CREATED_AT, Options::default()),
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
            ColumnFamilyDescriptor::new(MERKLE_RT, Options::default()),
            ColumnFamilyDescriptor::new(MERKLE_NODE, Options::default()),
            ColumnFamilyDescriptor::new(VALIDATOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(TX_HASHES, Options::default()),
            ColumnFamilyDescriptor::new(WITNESS_SIGS, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(MERKLE_ROOT, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(CTR_STATE, Options::default()),
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
