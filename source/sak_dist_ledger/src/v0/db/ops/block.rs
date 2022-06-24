use crate::{columns, Database};
use sak_kv_db::WriteBatch;
use sak_types::Block;

impl Database {
    pub(crate) async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, String> {
        let db = &self.ledger_db.db_instance;

        let cf_handle = match db.cf_handle(columns::VALIDATOR_SIG) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger colums {}",
                    columns::VALIDATOR_SIG
                ));
            }
        };

        let validator_sig = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => return Ok(None),
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::VALIDATOR_SIG,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(columns::TX_HASHES) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::TX_HASHES
                ));
            }
        };

        let get_cf_handle = db.get_cf(cf_handle, &block_hash);

        let tx_hashes = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                }
                None => {
                    return Ok(None);
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::TX_HASHES,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(columns::WITNESS_SIGS) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::WITNESS_SIGS
                ));
            }
        };

        let get_cf_handle = db.get_cf(cf_handle, &block_hash);

        let witness_sigs = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                }
                None => return Ok(None),
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::WITNESS_SIGS,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CREATED_AT
                ));
            }
        };

        let created_at = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => return Ok(None),
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::CREATED_AT,
                    err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(columns::BLOCK_HEIGHT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::BLOCK_HEIGHT
                ));
            }
        };

        let height = match db.get_cf(cf_handle, &block_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => return Ok(None),
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::BLOCK_HEIGHT,
                    err,
                ));
            }
        };

        let b = Block::new(
            validator_sig,
            tx_hashes,
            witness_sigs,
            created_at,
            height,
        );

        Ok(Some(b))
    }

    pub(crate) async fn get_block_hash_by_height(
        &self,
        block_height: String,
    ) -> Result<Option<String>, String> {
        let db = &self.ledger_db.db_instance;

        let cf_handle = match db.cf_handle(columns::BLOCK_HASH) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::BLOCK_HASH
                ));
            }
        };

        let block_hash = match db.get_cf(cf_handle, &block_height) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => return Ok(None),
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::BLOCK_HASH,
                    err,
                ));
            }
        };

        Ok(Some(block_hash))
    }

    pub(crate) async fn write_block(
        &self,
        block: &Block,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let mut batch = WriteBatch::default();

        let block_hash = block.get_hash();

        println!("write_block, hash: {}", block_hash);

        let cf_handle = match db.cf_handle(columns::VALIDATOR_SIG) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::VALIDATOR_SIG
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_validator_sig());

        let cf_handle = match db.cf_handle(columns::WITNESS_SIGS) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::WITNESS_SIGS
                ))
            }
        };

        let witness_sigs = match serde_json::to_string(block.get_witness_sigs())
        {
            Ok(v) => v,
            Err(err) => {
                return Err(format!(
                    "Cannot serialize {}, err: {}",
                    columns::WITNESS_SIGS,
                    err
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, witness_sigs);

        let cf_handle = match db.cf_handle(columns::TX_HASHES) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::TX_HASHES
                ))
            }
        };

        let transactions = match serde_json::to_string(&block.get_tx_hashes()) {
            Ok(v) => v,
            Err(err) => {
                return Err(format!(
                    "Cannot serialize {}, err: {}",
                    columns::TX_HASHES,
                    err
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, transactions);

        let cf_handle = match db.cf_handle(columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CREATED_AT
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_created_at());

        // put k : height && v : hash
        let cf_handle = match db.cf_handle(columns::BLOCK_HASH) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::BLOCK_HASH
                ))
            }
        };
        batch.put_cf(cf_handle, &block.get_height(), &block_hash);

        let cf_handle = match db.cf_handle(columns::BLOCK_HEIGHT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::BLOCK_HEIGHT
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.get_height());

        match db.write(batch) {
            Ok(_) => return Ok(block_hash.clone()),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }
}
