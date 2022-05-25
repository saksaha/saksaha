use super::{db, tx_columns};
use crate::blockchain::{blockchain::TxValue, TxHash, BlockValue};
use database::KeyValueDatabase;
use db::block_columns;
use logger::tinfo;
use rocksdb::{
    DBRawIteratorWithThreadMode, DBWithThreadMode, SingleThreaded, WriteBatch, ColumnFamily,
};
use sha3::{Digest, Sha3_256};

pub(crate) struct Ledger {
    ledger_db: KeyValueDatabase,
}

impl Ledger {
    pub(crate) async fn init(app_prefix: &String) -> Result<Ledger, String> {
        let ledger_db = match db::init_ledger_db(&app_prefix) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Could not initialize ledger db, err: {}",
                    err
                ));
            }
        };

        let ledger = Ledger { ledger_db };

        tinfo!("saksaha", "ledger", "Initialized Ledger (and ledger db)");

        Ok(ledger)
    }

    pub(crate) async fn write_tx(
        &self,
        tx_value: TxValue,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db;

        let mut batch = WriteBatch::default();

        let tx_hash = tx_value.get_hash();

        let cf_handle = match db.cf_handle(tx_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", tx_columns::CREATED_AT))
            }
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.created_at);

        let cf_handle = match db.cf_handle(tx_columns::DATA) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns {}", tx_columns::DATA)),
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.data);

        let cf_handle = match db.cf_handle(tx_columns::PI) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns {}", tx_columns::PI)),
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.pi);

        let cf_handle = match db.cf_handle(tx_columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", tx_columns::SIG_VEC))
            }
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.sig_vec);

        match db.write(batch) {
            Ok(_) => return Ok(tx_hash.hash),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }

    pub(crate) async fn read_tx(
        &self,
        tx_hash: &String,
    ) -> Result<TxValue, String> {
        let db = &self.ledger_db.db;

        let mut tx_value_result = vec![
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
                                "Invalid utf8 given, err: {}",
                                err,
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

        Ok(TxValue {
            created_at: tx_value_result[0].clone(),
            data: tx_value_result[1].clone(),
            sig_vec: tx_value_result[2].clone(),
            pi: tx_value_result[3].clone(),
        })
    }

    pub(crate) async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<BlockValue, String> {
        println!("got block_hash: {}", block_hash);

        let db = &self.ledger_db.db;

        let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::CREATED_AT));
            }
        };

        let created_at = match db.get_cf(cf_handle, block_hash) {
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
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::CREATED_AT, block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                        err: {}",
                    block_columns::CREATED_AT, err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::TX_POOL) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::TX_POOL));
            }
        };
        let get_cf_handle = db.get_cf(cf_handle, block_hash);

        let tx_pool = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::TX_POOL, block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                        err: {}",
                    block_columns::TX_POOL, err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::SIG_VEC));
            }
        };
        let get_cf_handle = db.get_cf(cf_handle, block_hash);

        let sig_vec = match get_cf_handle.as_ref() {
            Ok(val) => match val.as_ref() {
                Some(v) => {
                    let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                    th
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::SIG_VEC, block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                        err: {}",
                    block_columns::SIG_VEC, err,
                ));
            }
        };

        let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::HEIGHT));
            }
        };

        let height = match db.get_cf(cf_handle, block_hash) {
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
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        block_columns::HEIGHT, block_hash,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                        err: {}",
                    block_columns::HEIGHT, err,
                ));
            }
        };

        let bv = BlockValue {
            tx_pool,
            sig_vec,
            created_at,
            height,
        };
        println!("requested blockvalue: {:?}", &bv);
        Ok(bv)
    }

    pub(crate) async fn write_block(
        &self,
        block_value: BlockValue,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db;

        let mut batch = WriteBatch::default();

        let block_hash = block_value.get_hash();

        println!(
            "write_block(): created_at: {}, block_hash: {:?}",
            block_value.created_at, block_hash
        );

        let cf_handle = match db.cf_handle(block_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::CREATED_AT))
            }
        };
        batch.put_cf(cf_handle, &block_hash, block_value.created_at);

        let cf_handle = match db.cf_handle(block_columns::SIG_VEC) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns {}", block_columns::SIG_VEC)),
        };
        let ser_sig_vec = match serde_json::to_string(&block_value.sig_vec) {
            Ok(v) => v,
            Err(err) => return Err(format!("Cannot serialize {}, err: {}", block_columns::SIG_VEC, err)),
        };
        batch.put_cf(cf_handle, &block_hash, ser_sig_vec);

        let cf_handle = match db.cf_handle(block_columns::HEIGHT) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns {}", block_columns::HEIGHT)),
        };
        batch.put_cf(cf_handle, &block_hash, block_value.height);

        let cf_handle = match db.cf_handle(block_columns::TX_POOL) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns {}", block_columns::TX_POOL))
            }
        };
        let ser_tx_pool = match serde_json::to_string(&block_value.tx_pool) {
            Ok(v) => v,
            Err(err) => return Err(format!("Cannot serialize {}, err: {}", block_columns::TX_POOL, err)),
        };
        batch.put_cf(cf_handle, &block_hash, ser_tx_pool);

        match db.write(batch) {
            Ok(_) => return Ok(block_hash),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }

    pub fn iter(
        &self,
    ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>> {
        let iter = self.ledger_db.db.raw_iterator_cf(
            self.ledger_db
                .db
                .cf_handle(tx_columns::CREATED_AT)
                .unwrap(),
        );

        iter
    }
}


pub trait Hashing {
    fn get_hash(&self) -> String;
}

impl Hashing for BlockValue {
    fn get_hash(&self) -> String {
        let hash = {
            let mut h = Sha3_256::new();
            let v = match serde_json::to_value(&self) {
                Ok(v) => v,
                Err(err) => return format!("Failed to serialize self, err: {}", err),
            };
            h.update(v.to_string());
            h.finalize()
        };

        format!("{:x}", hash)
    }
}

impl Hashing for TxValue {
    fn get_hash(&self) -> String {
        let hash = {
            let mut h = Sha3_256::new();
            let v = match serde_json::to_value(&self) {
                Ok(v) => v,
                Err(err) => return format!("Failed to serialize self, err: {}", err),
            };
            h.update(v.to_string());
            h.finalize()
        };

        format!("{:x}", hash)
    }
}

#[cfg(test)]
pub(crate) mod for_test {
    use super::*;

    pub(crate) fn delete_tx(
        ledger: &Ledger,
        key: &String,
    ) -> Result<(), String> {
        let db = &ledger.ledger_db.db;
        let created_at_cf = match db.cf_handle(tx_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `crated_at`"))
            }
        };

        match db.delete_cf(created_at_cf, key) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Error deleting column family created_at, err: {}",
                    err,
                ));
            }
        }

        let data_cf = match db.cf_handle(tx_columns::DATA) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `DATA`")),
        };
        match db.delete_cf(data_cf, key) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Error deleting column family data_cf, err: {}",
                    err,
                ));
            }
        }

        let pi_cf = match db.cf_handle(tx_columns::PI) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `PI`")),
        };
        match db.delete_cf(pi_cf, key) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Error deleting column family pi, err: {}",
                    err,
                ));
            }
        }

        let sig_vec_cf = match db.cf_handle(tx_columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `SIG_VEC`"))
            }
        };
        match db.delete_cf(sig_vec_cf, key) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Error deleting column family sig_vec, err: {}",
                    err,
                ));
            }
        }

        Ok(())
    }
}
