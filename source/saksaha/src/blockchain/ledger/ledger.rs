use super::{db, ledger_columns};
use crate::blockchain::blockchain::TxValue;
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
            tx_value.created_at, tx_hash
        );

        let cf_handle = match db.cf_handle(db::ledger_columns::CREATED_AT) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `crated_at`"))
            }
        };
        batch.put_cf(cf_handle, &tx_hash, tx_value.created_at);

        let cf_handle = match db.cf_handle(db::ledger_columns::DATA) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `DATA`")),
        };
        batch.put_cf(cf_handle, &tx_hash, tx_value.data);

        let cf_handle = match db.cf_handle(db::ledger_columns::PI) {
            Some(h) => h,
            None => return Err(format!("Fail to open ledger columns `PI`")),
        };
        batch.put_cf(cf_handle, &tx_hash, tx_value.pi);

        let cf_handle = match db.cf_handle(db::ledger_columns::SIG_VEC) {
            Some(h) => h,
            None => {
                return Err(format!("Fail to open ledger columns `SIG_VEC`"))
            }
        };
        batch.put_cf(cf_handle, &tx_hash, tx_value.sig_vec);

        match db.write(batch) {
            Ok(_) => return Ok(tx_hash),
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

            tx_value_result[idx] = match db.get_cf(cf_handle, tx_hash) {
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
                            cfn, tx_hash,
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
}

pub(crate) fn get_hash<'a>(tx_val: &TxValue) -> String {
    let hash = {
        let mut h = Sha3_256::new();
        h.update(tx_val.created_at.clone());
        h.finalize()
    };

    format!("{:x}", hash)
}
