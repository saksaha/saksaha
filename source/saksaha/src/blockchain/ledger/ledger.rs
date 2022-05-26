use super::{db, ledger_columns};
use crate::blockchain::{blockchain::TxValue, TxHash};
use database::KeyValueDatabase;
use logger::tinfo;
use rocksdb::{
    DBRawIteratorWithThreadMode, DBWithThreadMode, SingleThreaded, WriteBatch,
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

        let tx_hash = get_hash(&tx_value);

        println!(
            "write_tx(): created_at: {}, tx_hash: {:?}",
            tx_value.created_at, tx_hash.hash
        );

        let cf_handle = match db.cf_handle(db::ledger_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `crated_at`"))
            }
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.created_at);

        let cf_handle = match db.cf_handle(db::ledger_columns::DATA) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `DATA`")),
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.data);

        let cf_handle = match db.cf_handle(db::ledger_columns::PI) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `PI`")),
        };
        batch.put_cf(cf_handle, &tx_hash.hash, tx_value.pi);

        let cf_handle = match db.cf_handle(db::ledger_columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `SIG_VEC`"))
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
            ledger_columns::CREATED_AT,
            ledger_columns::DATA,
            ledger_columns::SIG_VEC,
            ledger_columns::PI,
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

    pub fn iter(
        &self,
    ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>> {
        let iter = self.ledger_db.db.raw_iterator_cf(
            self.ledger_db
                .db
                .cf_handle(ledger_columns::CREATED_AT)
                .unwrap(),
        );

        iter
    }

    // pub fn destroy_ledger_db(&self) -> Result<(), String> {
    //     self.ledger_db.destroy()
    // }
}

pub(crate) fn get_hash<'a>(tx_val: &TxValue) -> TxHash {
    let hash = {
        let mut h = Sha3_256::new();
        h.update(tx_val.created_at.clone());
        h.finalize()
    };

    TxHash { hash: format!("{:x}", hash) }
}

pub(crate) mod for_test {
    use super::*;

    pub(crate) fn delete_tx(
        ledger: &Ledger,
        key: &String,
    ) -> Result<(), String> {
        let db = &ledger.ledger_db.db;
        let created_at_cf = match db.cf_handle(db::ledger_columns::CREATED_AT) {
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

        let data_cf = match db.cf_handle(db::ledger_columns::DATA) {
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

        let pi_cf = match db.cf_handle(db::ledger_columns::PI) {
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

        let sig_vec_cf = match db.cf_handle(db::ledger_columns::SIG_VEC) {
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
