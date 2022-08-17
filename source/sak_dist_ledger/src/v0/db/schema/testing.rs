use crate::LedgerError;
use crate::{cfs, LedgerDBSchema};
use sak_crypto::{Bls12, Hasher, Proof, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_kv_db::DB;
use sak_proofs::{get_mimc_params_1_to_2, verify_proof_1_to_2};
use sak_types::{
    MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx, TxCtrOp, TxHash,
    TxHeight, TxType, SN,
};
use type_extension::U8Arr32;

impl LedgerDBSchema {
    pub(crate) fn put_tx(&self, tx: &Tx) -> Result<String, LedgerError> {
        let mut batch = WriteBatch::default();

        let tx_hash = match tx {
            Tx::Mint(t) => self.batch_put_mint_tx(&mut batch, t)?,
            Tx::Pour(t) => self.batch_put_pour_tx(&mut batch, t)?,
        };

        self.db.write(batch)?;

        Ok(tx_hash)
    }

    pub(crate) fn delete_tx(
        &self,
        tx_hash: &String,
    ) -> Result<(), LedgerError> {
        // let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        self.batch_delete_tx_created_at(&mut batch, tx_hash)?;

        self.batch_delete_data(&mut batch, tx_hash)?;

        self.batch_delete_pi(&mut batch, tx_hash)?;

        self.batch_delete_author_sig(&mut batch, tx_hash)?;

        Ok(())
    }
}
