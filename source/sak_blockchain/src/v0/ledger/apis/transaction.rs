use crate::tx_columns;
use sak_types::{Hashable, Transaction};

use sak_kv_db::{KeyValueDatabase, WriteBatch};

#[inline]
pub(crate) async fn write_tx(
    ledger_db: &KeyValueDatabase,
    tx: Transaction,
) -> Result<String, String> {
    let db = &ledger_db.db;

    let mut batch = WriteBatch::default();
    let tx_hash = match tx.get_hash() {
        Ok(h) => h,
        Err(err) => {
            return Err(format!(
                "Could not get hash out of tx, critical error: {}",
                err
            ))
        }
    };

    let cf_handle = match db.cf_handle(tx_columns::CREATED_AT) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                tx_columns::CREATED_AT
            ))
        }
    };
    batch.put_cf(cf_handle, &tx_hash, tx.created_at);

    let cf_handle = match db.cf_handle(tx_columns::DATA) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                tx_columns::DATA
            ))
        }
    };
    batch.put_cf(cf_handle, &tx_hash, tx.data);

    let cf_handle = match db.cf_handle(tx_columns::PI) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                tx_columns::PI
            ))
        }
    };
    batch.put_cf(cf_handle, &tx_hash, tx.pi);

    let cf_handle = match db.cf_handle(tx_columns::SIG_VEC) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                tx_columns::SIG_VEC
            ))
        }
    };
    batch.put_cf(cf_handle, &tx_hash, tx.signature);

    let cf_handle = match db.cf_handle(tx_columns::CONTRACT) {
        Some(h) => h,
        None => {
            return Err(format!(
                "Fail to open ledger columns {}",
                tx_columns::CONTRACT
            ))
        }
    };
    batch.put_cf(cf_handle, &tx_hash, tx.contract);

    match db.write(batch) {
        Ok(_) => return Ok(tx_hash),
        Err(err) => {
            return Err(format!("Fail to write on ledger db, err: {}", err))
        }
    }
}

#[inline]
pub(crate) async fn read_tx(
    ledger_db: &KeyValueDatabase,
    tx_hash: &String,
) -> Result<Transaction, String> {
    let db = &ledger_db.db;

    let mut tx_value_result = vec![
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];

    let tx_values_col = vec![
        tx_columns::CREATED_AT,
        tx_columns::DATA,
        tx_columns::SIG_VEC,
        tx_columns::PI,
        tx_columns::CONTRACT,
    ];

    let tx_values_it_map = tx_values_col.iter().map(|cf_name| cf_name);

    for (idx, cfn) in tx_values_it_map.enumerate() {
        let cf_handle = match db.cf_handle(cfn) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", cfn));
            }
        };
        tx_value_result[idx] = match db.get_cf(cf_handle, &tx_hash) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}, {}, {}",
                            err, idx, v
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        cfn, &tx_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                        err: {}",
                    cfn, err,
                ));
            }
        };
    }

    Ok(Transaction {
        created_at: tx_value_result[0].clone(),
        data: tx_value_result[1].as_bytes().to_vec(),
        signature: tx_value_result[2].clone(),
        pi: tx_value_result[3].clone(),
        contract: tx_value_result[4].as_bytes().to_vec(),
    })
}
