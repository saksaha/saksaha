use crate::BoxedError;
use sak_fs::FS;
use sak_kv_db::{
    ColumnFamilyDescriptor, DBRawIteratorWithThreadMode, DBWithThreadMode,
    KeyValueDatabase, Options, SingleThreaded, WriteBatch,
};
use sak_types::{Hashable, Transaction};

use super::tx_columns;

pub(crate) struct TxDB {
    kv_db: KeyValueDatabase,
}

impl TxDB {
    pub(crate) fn init(app_prefix: &String) -> Result<TxDB, BoxedError> {
        let tx_db_path = {
            let app_path = FS::create_or_get_app_path(app_prefix)?;
            let db_path = { app_path.join("db").join("tx") };

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let cf_descriptors = TxDB::make_cf_descriptors();

        let kv_db =
            match KeyValueDatabase::new(tx_db_path, options, cf_descriptors) {
                Ok(d) => d,
                Err(err) => {
                    return Err(format!(
                        "Error initializing key value database, err: {}",
                        err
                    )
                    .into());
                }
            };

        let d = TxDB { kv_db };

        Ok(d)
    }

    fn make_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        let columns = vec![
            (tx_columns::TX_HASH, Options::default()),
            (tx_columns::PI, Options::default()),
            (tx_columns::SIG_VEC, Options::default()),
            (tx_columns::CREATED_AT, Options::default()),
            (tx_columns::DATA, Options::default()),
            (tx_columns::CONTRACT, Options::default()),
        ];

        let cf = columns
            .into_iter()
            .map(|(col_name, options)| {
                ColumnFamilyDescriptor::new(col_name, options)
            })
            .collect();

        cf
    }

    pub(crate) async fn write_tx(
        &self,
        tx: Transaction,
    ) -> Result<String, String> {
        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        let tx_hash = match tx.get_hash() {
            Ok(hash) => hash,
            Err(_) => return Err(format!("Failed to get hash from tx_value")),
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

    pub(crate) async fn read_tx(
        &self,
        // ledger_db: &KeyValueDatabase,
        tx_hash: &String,
    ) -> Result<Transaction, String> {
        // let db = &ledger_db.db;
        let db = &self.kv_db.db_instance;

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

        Ok(Transaction {
            created_at: tx_value_result[0].clone(),
            data: tx_value_result[1].as_bytes().to_vec(),
            signature: tx_value_result[2].clone(),
            pi: tx_value_result[3].clone(),
            contract: tx_value_result[4].as_bytes().to_vec(),
        })
    }

    // for testing
    pub fn delete_tx(&self, key: &String) -> Result<(), String> {
        // let db = &ledger.ledger_db.db;
        let db = &self.kv_db.db_instance;

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

    pub fn iter(
        &self,
    ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>> {
        let db = &self.kv_db.db_instance;

        let iter =
            db.raw_iterator_cf(db.cf_handle(tx_columns::CREATED_AT).unwrap());

        iter
    }
}
