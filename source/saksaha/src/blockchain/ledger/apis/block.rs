use crate::blockchain::{ledger::block_columns, Block, Hashable};
use database::KeyValueDatabase;
use log::debug;
use rocksdb::WriteBatch;

#[inline]
pub(crate) async fn get_block(
    ledger_db: &KeyValueDatabase,
    block_hash: &String,
) -> Result<Block, String> {
    let db = &ledger_db.db;

    let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::CREATED_AT
            ));
        }
    };

    let created_at = match db.get_cf(cf_handle, &block_hash) {
        Ok(val) => match val {
            Some(v) => match std::str::from_utf8(&v) {
                Ok(vs) => vs.to_string(),
                Err(err) => {
                    return Err(format!("Invalid utf8 given, err: {}", err,));
                }
            },
            None => {
                return Err(format!(
                    "No matched value with tx_hash in {}, {}",
                    block_columns::CREATED_AT,
                    &block_hash,
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                block_columns::CREATED_AT,
                err,
            ));
        }
    };

    let cf_handle = match db.cf_handle(block_columns::TX_POOL) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::TX_POOL
            ));
        }
    };
    let get_cf_handle = db.get_cf(cf_handle, &block_hash);

    let transactions = match get_cf_handle.as_ref() {
        Ok(val) => match val.as_ref() {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                th
            }
            None => {
                return Err(format!(
                    "No matched value with tx_hash in {}, {}",
                    block_columns::TX_POOL,
                    &block_hash,
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                block_columns::TX_POOL,
                err,
            ));
        }
    };

    let cf_handle = match db.cf_handle(block_columns::SIG_VEC) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::SIG_VEC
            ));
        }
    };
    let get_cf_handle = db.get_cf(cf_handle, &block_hash);

    let signatures = match get_cf_handle.as_ref() {
        Ok(val) => match val.as_ref() {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                th
            }
            None => {
                return Err(format!(
                    "No matched value with tx_hash in {}, {}",
                    block_columns::SIG_VEC,
                    &block_hash,
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                block_columns::SIG_VEC,
                err,
            ));
        }
    };

    let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::HEIGHT
            ));
        }
    };

    let height = match db.get_cf(cf_handle, &block_hash) {
        Ok(val) => match val {
            Some(v) => match std::str::from_utf8(&v) {
                Ok(vs) => vs.to_string(),
                Err(err) => {
                    return Err(format!("Invalid utf8 given, err: {}", err,));
                }
            },
            None => {
                return Err(format!(
                    "No matched value with tx_hash in {}, {}",
                    block_columns::HEIGHT,
                    &block_hash,
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                block_columns::HEIGHT,
                err,
            ));
        }
    };

    let b = Block {
        transactions,
        signatures,
        created_at,
        height,
    };

    Ok(b)
}

#[inline]
pub(crate) async fn write_block(
    ledger_db: &KeyValueDatabase,
    block: Block,
) -> Result<String, String> {
    let db = &ledger_db.db;

    let mut batch = WriteBatch::default();

    let block_hash = match block.get_hash() {
        Ok(hash) => hash,
        Err(_) => return Err(format!("Failed to get hash from block_value")),
    };

    debug!(
        "write_block(): created_at: {}, block_hash: {:?}",
        block.created_at, block_hash
    );

    {
        let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    block_columns::CREATED_AT
                ))
            }
        };

        batch.put_cf(cf_handle, &block_hash, block.created_at);
    }

    let cf_handle = match db.cf_handle(block_columns::SIG_VEC) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::SIG_VEC
            ))
        }
    };

    let ser_signatures = match serde_json::to_string(&block.signatures) {
        Ok(v) => v,
        Err(err) => {
            return Err(format!(
                "Cannot serialize {}, err: {}",
                block_columns::SIG_VEC,
                err
            ))
        }
    };

    batch.put_cf(cf_handle, &block_hash, ser_signatures);

    let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::HEIGHT
            ))
        }
    };

    batch.put_cf(cf_handle, &block_hash, block.height);

    let cf_handle = match db.cf_handle(block_columns::TX_POOL) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                block_columns::TX_POOL
            ))
        }
    };

    let ser_transactions = match serde_json::to_string(&block.transactions) {
        Ok(v) => v,
        Err(err) => {
            return Err(format!(
                "Cannot serialize {}, err: {}",
                block_columns::TX_POOL,
                err
            ))
        }
    };

    batch.put_cf(cf_handle, &block_hash, ser_transactions);

    match db.write(batch) {
        Ok(_) => return Ok(block_hash),
        Err(err) => {
            return Err(format!("Fail to write on ledger db, err: {}", err))
        }
    }
}
