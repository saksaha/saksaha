use crate::{LedgerDB, LedgerError};
use sak_kv_db::{WriteBatch, DB};
use sak_types::{Tx, TxType, WASM_MAGIC_NUMBER};

impl LedgerDB {
    #[cfg(test)]
    pub(crate) fn put_tx(&self, tx: &Tx) -> Result<String, LedgerError> {
        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        self._batch_put_tx(db, &mut batch, tx)
    }

    pub(crate) async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        let db = &self.kv_db.db_instance;

        let created_at = self
            .schema
            .get_created_at(db, tx_hash)?
            .ok_or("created_at does not exist")?;

        let data = self
            .schema
            .get_data(db, tx_hash)?
            .ok_or("data does not exist")?;

        let author_sig = self
            .schema
            .get_author_sig(db, tx_hash)?
            .ok_or("author_sig does not exist")?;

        let pi = self
            .schema
            .get_pi(db, tx_hash)?
            .ok_or("pi does not exist")?;

        let ctr_addr = self
            .schema
            .get_ctr_addr(db, tx_hash)?
            .ok_or("ctr_addr does not exist")?;

        let tx = Tx::new(created_at, data, author_sig, pi, Some(ctr_addr));

        Ok(Some(tx))
    }

    pub(super) fn _batch_put_tx(
        &self,
        db: &DB,
        batch: &mut WriteBatch,
        tx: &Tx,
    ) -> Result<String, LedgerError> {
        let tx_hash = tx.get_hash();

        self.schema.batch_put_created_at(
            db,
            batch,
            tx_hash,
            tx.get_created_at(),
        )?;

        self.schema
            .batch_put_data(db, batch, tx_hash, tx.get_data())?;

        self.schema.batch_put_pi(db, batch, tx_hash, tx.get_pi())?;

        self.schema.batch_put_author_sig(
            db,
            batch,
            tx_hash,
            tx.get_author_sig(),
        )?;

        self.schema.batch_put_ctr_addr(
            db,
            batch,
            tx_hash,
            tx.get_ctr_addr(),
        )?;

        match tx.get_type() {
            TxType::ContractDeploy => {
                self.schema.batch_put_tx_hash(
                    db,
                    batch,
                    tx.get_ctr_addr(),
                    tx_hash,
                )?;
            }
            TxType::ContractCall => {}
            TxType::Plain => {}
        }

        Ok(tx_hash.clone())
    }
}

pub mod testing {
    use super::*;

    impl LedgerDB {
        // pub fn iter(
        //     &self,
        // ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>>
        // {
        //     let db = &self.kv_db.db_instance;

        //     let iter =
        //         db.raw_iterator_cf(db.cf_handle(columns::CREATED_AT).unwrap());

        //     iter
        // }

        pub fn delete_tx(&self, tx_hash: &String) -> Result<(), LedgerError> {
            let db = &self.kv_db.db_instance;

            let mut batch = WriteBatch::default();

            self.schema
                .batch_delete_created_at(db, &mut batch, tx_hash)?;

            self.schema.batch_delete_data(db, &mut batch, tx_hash)?;

            self.schema.batch_delete_pi(db, &mut batch, tx_hash)?;

            self.schema
                .batch_delete_author_sig(db, &mut batch, tx_hash)?;

            Ok(())
        }
    }
}
