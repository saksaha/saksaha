use super::{apis, db, tx_columns};
use crate::blockchain::{blockchain::TxValue, BlockValue, Hash};
use database::KeyValueDatabase;
use db::block_columns;
use logger::tinfo;
use rocksdb::{DBRawIteratorWithThreadMode, DBWithThreadMode, SingleThreaded};
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
    ) -> Result<Hash, String> {
        apis::write_tx(&self.ledger_db, tx_value).await
    }

    pub(crate) async fn read_tx(
        &self,
        tx_hash: &Hash,
    ) -> Result<TxValue, String> {
        apis::read_tx(&self.ledger_db, tx_hash).await
    }

    pub(crate) async fn get_block(
        &self,
        block_hash: &Hash,
    ) -> Result<BlockValue, String> {
        apis::get_block(&self.ledger_db, block_hash).await
    }

    pub(crate) async fn write_block(
        &self,
        block_value: BlockValue,
    ) -> Result<Hash, String> {
        apis::write_block(&self.ledger_db, block_value).await
    }

    pub fn iter(
        &self,
    ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>> {
        let iter = self.ledger_db.db.raw_iterator_cf(
            self.ledger_db.db.cf_handle(tx_columns::CREATED_AT).unwrap(),
        );

        iter
    }
}

pub trait Hashable {
    fn get_hash(&self) -> Result<Hash, String>;
}

impl Hashable for BlockValue {
    fn get_hash(&self) -> Result<Hash, String> {
        let hash = {
            let mut h = Sha3_256::new();
            let v = match serde_json::to_value(&self) {
                Ok(v) => v,
                Err(err) => {
                    return Err(format!(
                        "Failed to serialize self, err: {}",
                        err
                    ))
                }
            };
            h.update(v.to_string());
            h.finalize()
        };

        Ok(Hash {
            hash: format!("{:x}", hash),
        })
    }
}

impl Hashable for TxValue {
    fn get_hash(&self) -> Result<Hash, String> {
        let hash = {
            let mut h = Sha3_256::new();
            let v = match serde_json::to_value(&self) {
                Ok(v) => v,
                Err(err) => {
                    return Err(format!(
                        "Failed to serialize self, err: {}",
                        err
                    ))
                }
            };
            h.update(v.to_string());
            h.finalize()
        };

        Ok(Hash {
            hash: format!("{:x}", hash),
        })
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
