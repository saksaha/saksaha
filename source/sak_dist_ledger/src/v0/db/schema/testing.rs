use crate::{LedgerDB, LedgerError};
use sak_kv_db::WriteBatch;
use sak_types::Tx;

impl LedgerDB {
    // #[cfg(test)]
    // pub(crate) fn put_tx(
    //     &self,
    //     tx: &Tx,
    //     // cm_idx_count: &mut u128,
    // ) -> Result<String, LedgerError> {
    //     let mut batch = WriteBatch::default();

    //     let tx_hash = match tx {
    //         Tx::Mint(t) => {
    //             self.batch_put_mint_tx(
    //                 &mut batch, t,
    //                 // cm_idx_count
    //             )?
    //         }
    //         Tx::Pour(t) => {
    //             self.batch_put_pour_tx(
    //                 &mut batch, t,
    //                 // cm_idx_count
    //             )?
    //         }
    //     };

    //     self.db.write(batch)?;

    //     Ok(tx_hash)
    // }

    pub(crate) fn delete_tx(&self, tx_hash: &String) -> Result<(), LedgerError> {
        let mut batch = WriteBatch::default();

        self.batch_delete_tx_created_at(&mut batch, tx_hash)?;

        self.batch_delete_data(&mut batch, tx_hash)?;

        self.batch_delete_pi(&mut batch, tx_hash)?;

        self.batch_delete_author_sig(&mut batch, tx_hash)?;

        Ok(())
    }
}
