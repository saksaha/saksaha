use super::{apis, db, tx_columns};
use crate::{Block, Transaction};
use database::KeyValueDatabase;
use logger::tinfo;
use rocksdb::{DBRawIteratorWithThreadMode, DBWithThreadMode, SingleThreaded};

pub struct Ledger {
    ledger_db: KeyValueDatabase,
}

impl Ledger {
    pub async fn init(app_prefix: &String) -> Result<Ledger, String> {
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

    pub async fn write_tx(&self, tx: Transaction) -> Result<String, String> {
        apis::write_tx(&self.ledger_db, tx).await
    }

    pub async fn read_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Transaction, String> {
        apis::read_tx(&self.ledger_db, tx_hash).await
    }

    pub async fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Block, String> {
        apis::get_block(&self.ledger_db, block_hash).await
    }

    pub async fn write_block(&self, block: Block) -> Result<String, String> {
        apis::write_block(&self.ledger_db, block).await
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

#[cfg(test)]
pub mod ledger_for_test {
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
