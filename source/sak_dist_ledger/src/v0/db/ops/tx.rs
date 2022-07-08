use crate::{get_tx_type, LedgerDB, LedgerError};
use sak_kv_db::{WriteBatch, DB};
use sak_types::{Tx, TxType};

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

        let tx_height = self
            .schema
            .get_tx_height(db, tx_hash)?
            .ok_or("tx_height does not exist")?;

        let cm = self
            .schema
            .get_cm(db, tx_hash)?
            .ok_or("cm does not exist")?;

        let v = self.schema.get_v(db, tx_hash)?.ok_or("v does not exist")?;
        let k = self.schema.get_k(db, tx_hash)?.ok_or("k does not exist")?;
        let s = self.schema.get_s(db, tx_hash)?.ok_or("s does not exist")?;
        let sn_1 = self
            .schema
            .get_sn_1(db, tx_hash)?
            .ok_or("sn_1 does not exist")?;
        let sn_2 = self
            .schema
            .get_sn_2(db, tx_hash)?
            .ok_or("sn_2 does not exist")?;
        let cm_1 = self
            .schema
            .get_cm_1(db, tx_hash)?
            .ok_or("cm_1 does not exist")?;
        let cm_2 = self
            .schema
            .get_cm_2(db, tx_hash)?
            .ok_or("cm_2 does not exist")?;
        let rt = self
            .schema
            .get_rt(db, tx_hash)?
            .ok_or("rt does not exist")?;

        let tx = Tx::new(
            created_at,
            data,
            author_sig,
            pi,
            ctr_addr,
            tx_hash.to_owned(),
            cm,
            v,
            k,
            s,
            sn_1,
            sn_2,
            cm_1,
            cm_2,
            rt,
            tx_height,
        );

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

        self.schema.batch_put_tx_height(
            db,
            batch,
            tx_hash,
            tx.get_tx_height(),
        )?;

        self.schema.batch_put_tx_hash_by_height(
            db,
            batch,
            tx.get_tx_height(),
            tx_hash,
        )?;

        self.schema.batch_put_cm(db, batch, tx_hash, tx.get_cm())?;

        self.schema.batch_put_cm_by_height(
            db,
            batch,
            tx.get_tx_height(),
            tx.get_cm(),
        )?;

        // self.schema.batch_put_cm(db, batch, tx_hash, tx.get_cm())?;

        match get_tx_type(tx.get_ctr_addr(), tx.get_data()) {
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

    pub(crate) async fn get_latest_tx_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        let db = &self.kv_db.db_instance;

        let height = self.schema.get_latest_tx_height(db)?;

        Ok(height)
    }

    pub(crate) async fn get_tx_hash_by_height(
        &self,
        height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_tx_hash_by_height(db, height)
    }

    pub(crate) async fn get_cm_by_height(
        &self,
        height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_cm_by_height(db, height)
    }

    pub(crate) async fn get_rt(
        &self,
        tx_hash: &String,
    ) -> Result<Option<String>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_rt(db, tx_hash)
    }
}

pub mod testing {
    use super::*;

    impl LedgerDB {
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
