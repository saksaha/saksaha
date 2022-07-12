use crate::{LedgerDB, LedgerDBSchema, LedgerError};
use sak_kv_db::{KeyValueDatabase, WriteBatch, DB};
use sak_types::{
    MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx, TxCandidate, TxCtrOp,
};

impl LedgerDB {
    #[cfg(test)]
    pub(crate) fn put_tx(&self, tx: &Tx) -> Result<String, LedgerError> {
        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        match tx {
            Tx::Mint(t) => batch_put_mint_tx(db, &self.schema, &mut batch, t),
            Tx::Pour(t) => batch_put_pour_tx(db, &self.schema, &mut batch, t),
        }
    }

    pub(crate) async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        let db = &self.kv_db.db_instance;

        let tx_type = self
            .schema
            .get_tx_type(db, tx_hash)?
            .ok_or("tx type should exist")?;

        let tx = match tx_type.as_ref() {
            "mint" => get_mint_tx(db, &self.schema, tx_hash),
            "pour" => get_pour_tx(db, &self.schema, tx_hash),
        }?;

        Ok(Some(tx))
    }

    pub(crate) async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        let mut ret = vec![];
        for tx_hash in tx_hashes {
            match self.get_tx(tx_hash).await? {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
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

    pub(crate) async fn get_merkle_node(
        &self,
        location: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_merkle_node(db, location)
    }

    pub(crate) async fn get_merkle_rt(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_merkle_rt(db, tx_hash)
    }
}

fn get_mint_tx(
    db: &DB,
    schema: &LedgerDBSchema,
    tx_hash: &String,
) -> Result<Tx, LedgerError> {
    let created_at = schema
        .get_created_at(db, tx_hash)?
        .ok_or("created_at does not exist")?;

    let data = schema.get_data(db, tx_hash)?.ok_or("data does not exist")?;

    let author_sig = schema
        .get_author_sig(db, tx_hash)?
        .ok_or("author_sig does not exist")?;

    let ctr_addr = schema.get_ctr_addr(db, tx_hash)?;

    let cm = schema.get_cm(db, tx_hash)?.ok_or("cm should exist")?;

    let v = schema.get_v(db, tx_hash)?.ok_or("v should exist")?;

    let k = schema.get_k(db, tx_hash)?.ok_or("k should exist")?;

    let s = schema.get_s(db, tx_hash)?.ok_or("s shoudl exist")?;

    let tx_height = schema
        .get_tx_height(db, tx_hash)?
        .ok_or("tx_height does not exist")?;

    let tx_candidate = MintTxCandidate::new(
        created_at, data, author_sig, ctr_addr, cm, v, k, s,
    );

    let tx = Tx::Mint(MintTx::new(tx_candidate, tx_height));

    Ok(tx)
}

fn get_pour_tx(
    db: &DB,
    schema: &LedgerDBSchema,
    tx_hash: &String,
) -> Result<Tx, LedgerError> {
    let created_at = schema
        .get_created_at(db, tx_hash)?
        .ok_or("created_at does not exist")?;

    let data = schema.get_data(db, tx_hash)?.ok_or("data does not exist")?;

    let author_sig = schema
        .get_author_sig(db, tx_hash)?
        .ok_or("author_sig does not exist")?;

    let ctr_addr = schema.get_ctr_addr(db, tx_hash)?;

    let pi = schema.get_pi(db, tx_hash)?.ok_or("pi should exist")?;

    let sn_1 = schema.get_sn_1(db, tx_hash)?.ok_or("sn_1 should exist")?;

    let sn_2 = schema.get_cm_2(db, tx_hash)?.ok_or("sn_2 should exist")?;

    let cm_1 = schema.get_cm_1(db, tx_hash)?.ok_or("cm_1 should exist")?;

    let cm_2 = schema.get_cm_2(db, tx_hash)?.ok_or("cm_2 should exist")?;

    let merkle_rt = schema
        .get_merkle_rt(db, tx_hash)?
        .ok_or("merkle_root should exist")?;

    let tx_candidate = PourTxCandidate::new(
        created_at, data, author_sig, ctr_addr, pi, sn_1, sn_2, cm_1, cm_2,
        merkle_rt,
    );

    let tx_height = schema
        .get_tx_height(db, tx_hash)?
        .ok_or("tx_height does not exist")?;

    let tx = Tx::Pour(PourTx::new(tx_candidate, tx_height));

    Ok(tx)
}

fn batch_put_mint_tx(
    db: &DB,
    schema: &LedgerDBSchema,
    batch: &mut WriteBatch,
    tx: &MintTx,
) -> Result<String, LedgerError> {
    let tc = tx.tx_candidate;

    let tx_hash = tc.get_tx_hash();

    schema.batch_put_cm(db, batch, tx_hash, &tc.cm)?;

    schema.batch_put_cm_by_height(db, batch, &tx.tx_height, &tc.cm)?;

    schema.batch_put_created_at(db, batch, tx_hash, &tc.created_at)?;

    schema.batch_put_data(db, batch, tx_hash, &tc.data)?;

    schema.batch_put_author_sig(db, batch, tx_hash, &tc.author_sig)?;

    schema.batch_put_ctr_addr(db, batch, tx_hash, &tc.ctr_addr)?;

    schema.batch_put_tx_height(db, batch, tx_hash, &tx.tx_height)?;

    schema.batch_put_tx_hash_by_height(db, batch, &tx.tx_height, tx_hash)?;

    let tx_ctr_op = tc.get_ctr_op();

    match tx_ctr_op {
        TxCtrOp::ContractDeploy => {
            schema.batch_put_tx_hash(db, batch, &tc.ctr_addr, tx_hash)?;
        }
        TxCtrOp::ContractCall => {}
        TxCtrOp::None => {}
    }

    Ok(tx_hash.clone())
}

fn batch_put_pour_tx(
    db: &DB,
    schema: &LedgerDBSchema,
    batch: &mut WriteBatch,
    tx: &PourTx,
) -> Result<String, LedgerError> {
    let tc = tx.tx_candidate;

    let tx_hash = tc.get_tx_hash();

    schema.batch_put_created_at(db, batch, tx_hash, &tc.created_at)?;

    schema.batch_put_data(db, batch, tx_hash, &tc.data)?;

    schema.batch_put_author_sig(db, batch, tx_hash, &tc.author_sig)?;

    schema.batch_put_ctr_addr(db, batch, tx_hash, &tc.ctr_addr)?;

    schema.batch_put_tx_height(db, batch, tx_hash, &tx.tx_height)?;

    schema.batch_put_tx_hash_by_height(db, batch, &tx.tx_height, tx_hash)?;

    schema.batch_put_sn_1(db, batch, tx_hash, &tc.sn_1)?;

    schema.batch_put_sn_2(db, batch, tx_hash, &tc.sn_2)?;

    schema.batch_put_cm_1(db, batch, tx_hash, &tc.cm_1)?;

    schema.batch_put_cm_2(db, batch, tx_hash, &tc.cm_2)?;

    schema.batch_put_merkle_rt(db, batch, tx_hash, &tc.merkle_rt)?;

    let tx_ctr_op = tc.get_ctr_op();

    match tx_ctr_op {
        TxCtrOp::ContractDeploy => {
            schema.batch_put_tx_hash(db, batch, &tc.ctr_addr, tx_hash)?;
        }
        TxCtrOp::ContractCall => {}
        TxCtrOp::None => {}
    }

    Ok(tx_hash.clone())
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
