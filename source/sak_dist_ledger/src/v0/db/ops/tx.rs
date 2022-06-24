use crate::{columns, Database};
use sak_kv_db::{
    DBRawIteratorWithThreadMode, DBWithThreadMode, KeyValueDatabase,
    SingleThreaded, WriteBatch,
};
use sak_types::Tx;

impl Database {
    pub(crate) async fn write_tx(&self, tx: &Tx) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let mut batch = WriteBatch::default();

        let tx_hash = tx.get_hash();

        let cf_handle = match db.cf_handle(columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CREATED_AT
                ))
            }
        };
        batch.put_cf(cf_handle, tx_hash, tx.get_created_at());

        let cf_handle = match db.cf_handle(columns::DATA) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::DATA
                ))
            }
        };
        batch.put_cf(cf_handle, tx_hash, tx.get_data());

        let cf_handle = match db.cf_handle(columns::PI) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::PI
                ))
            }
        };
        batch.put_cf(cf_handle, tx_hash, tx.get_pi());

        let cf_handle = match db.cf_handle(columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::SIG_VEC
                ))
            }
        };
        batch.put_cf(cf_handle, tx_hash, tx.get_signature());

        let cf_handle = match db.cf_handle(columns::CONTRACT_ADDR) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CONTRACT_ADDR,
                ))
            }
        };
        batch.put_cf(cf_handle, tx_hash, tx.get_contract_addr());

        //

        match db.write(batch) {
            Ok(_) => return Ok(tx_hash.clone()),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }

    pub(crate) async fn read_tx(&self, tx_hash: &String) -> Result<Tx, String> {
        let db = &self.ledger_db.db_instance;

        let mut tx_value_result = vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];

        let tx_values_col = vec![
            columns::CREATED_AT,
            columns::DATA,
            columns::SIG_VEC,
            columns::PI,
            columns::CONTRACT,
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

        Ok(Tx::new(
            tx_value_result[0].clone(),
            tx_value_result[1].as_bytes().to_vec(),
            tx_value_result[2].clone(),
            tx_value_result[3].clone(),
            Some(tx_value_result[4].as_bytes().to_vec()),
        ))
    }

    // for testing
    pub fn delete_tx(&self, key: &String) -> Result<(), String> {
        let db = &self.ledger_db.db_instance;

        let created_at_cf = match db.cf_handle(columns::CREATED_AT) {
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

        let data_cf = match db.cf_handle(columns::DATA) {
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

        let pi_cf = match db.cf_handle(columns::PI) {
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

        let sig_vec_cf = match db.cf_handle(columns::SIG_VEC) {
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

pub mod testing {
    use super::*;

    impl Database {
        pub fn iter(
            &self,
        ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>>
        {
            let db = &self.ledger_db.db_instance;

            let iter =
                db.raw_iterator_cf(db.cf_handle(columns::CREATED_AT).unwrap());

            iter
        }
    }
}
