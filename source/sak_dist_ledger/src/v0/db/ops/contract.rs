use crate::{LedgerDB, LedgerError};
use sak_contract_std::Storage;
use sak_kv_db::WriteBatch;

impl LedgerDB {
    // pub(crate) async fn get_ctr_data_by_ctr_addr(
    //     &self,
    //     ctr_addr: &String,
    // ) -> Result<Option<Vec<u8>>, LedgerError> {
    //     let db = &self.kv_db.db_instance;

    //     let tx_hash = self
    //         .schema
    //         .get_tx_hash(db, ctr_addr)?
    //         .ok_or("ctr data does not exist")?;

    //     let ctr_data = self
    //         .schema
    //         .get_data(db, &tx_hash)?
    //         .ok_or("data does not exist")?;

    //     Ok(Some(ctr_data))
    // }

    // pub(crate) fn get_ctr_state(
    //     &self,
    //     ctr_addr: &String,
    // ) -> Result<Option<Storage>, LedgerError> {
    //     let db = &self.kv_db.db_instance;

    //     let ctr_state = self
    //         .schema
    //         .get_ctr_state(db, &ctr_addr)?
    //         .ok_or("ctr state does not exist")?;

    //     let storage: Storage = serde_json::from_slice(&ctr_state)?;

    //     Ok(Some(storage))
    // }

    // #[cfg(test)]
    // pub(crate) async fn batch_put_ctr_state(
    //     &self,

    //     ctr_addr: &String,
    //     ctr_state: &String,
    // ) -> Result<String, LedgerError> {
    //     let db = &self.kv_db.db_instance;

    //     let mut batch = WriteBatch::default();

    //     self.schema
    //         .batch_put_ctr_state(db, &mut batch, &ctr_addr, ctr_state)?;

    //     db.write(batch)?;

    //     return Ok("".to_string().clone());
    // }
}
