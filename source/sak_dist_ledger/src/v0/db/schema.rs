use crate::LedgerError;
use sak_kv_db::{
    ColumnFamily, ColumnFamilyDescriptor, KeyValueDatabase, Options,
    WriteBatch, DB,
};

//
const TX_HASH: &str = "tx_hash";

//
const PI: &str = "pi";

//
const SIG_VEC: &str = "sig_vec";

//
const CREATED_AT: &str = "created_at";

//
const DATA: &str = "data";

//
const CTR_ADDR: &str = "ctr_addr";

//
const VALIDATOR_SIG: &str = "validator_sig";

//
const TX_HASHES: &str = "tx_hashes";

//
const WITNESS_SIGS: &str = "witness_sigs";

//
const BLOCK_HEIGHT: &str = "block_height";

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

        match db.get_cf(cf, block_hash)? {
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

        match db.get_cf(cf, block_hash)? {
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
        block_hash: &String,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = make_cf_handle(db, WITNESS_SIGS)?;

        match db.get_cf(cf, block_hash)? {
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
        block_hash: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, CREATED_AT)?;

        match db.get_cf(cf, block_hash)? {
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
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, CREATED_AT)?;

        match db.get_cf(cf, block_hash)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_hash(
        &self,
        db: &DB,
        block_height: &String,
    ) -> Result<Option<String>, LedgerError> {
        let cf = make_cf_handle(db, BLOCK_HASH)?;

        match db.get_cf(cf, block_height)? {
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
        ctr_addr: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, DATA)?;

        match db.get_cf(cf, ctr_addr)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_ctr_state(
        &self,
        db: &DB,
        ctr_addr: &String,
        field_name: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = make_cf_handle(db, CTR_STATE)?;

        let state_key = format!("{}:{}", ctr_addr, field_name);

        match db.get_cf(cf, state_key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn batch_put_validator_sig(
        &self,
        db: &DB,
        batch: WriteBatch,
        block_hash: &String,
        validator_sig: &String,
    ) -> Result<(), LedgerError> {
        let cf = make_cf_handle(db, VALIDATOR_SIG)?;

        batch.put_cf(cf, block_hash, validator_sig);

        Ok(())
    }

    pub(crate) fn make_cf_descriptors(&self) -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(TX_HASH, Options::default()),
            ColumnFamilyDescriptor::new(PI, Options::default()),
            ColumnFamilyDescriptor::new(SIG_VEC, Options::default()),
            ColumnFamilyDescriptor::new(CREATED_AT, Options::default()),
            ColumnFamilyDescriptor::new(DATA, Options::default()),
            ColumnFamilyDescriptor::new(CTR_ADDR, Options::default()),
            ColumnFamilyDescriptor::new(VALIDATOR_SIG, Options::default()),
            ColumnFamilyDescriptor::new(TX_HASHES, Options::default()),
            ColumnFamilyDescriptor::new(WITNESS_SIGS, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HEIGHT, Options::default()),
            ColumnFamilyDescriptor::new(BLOCK_HASH, Options::default()),
            ColumnFamilyDescriptor::new(CTR_STATE, Options::default()),
        ]
    }
}

fn make_cf_handle<'a>(
    db: &'a DB,
    col_name: &'static str,
) -> Result<&'a ColumnFamily, String> {
    let cf_handle = match db.cf_handle(col_name) {
        Some(h) => h,
        None => {
            return Err(format!("Fail to open ledger colums {}", col_name,));
        }
    };

    Ok(cf_handle)
}
