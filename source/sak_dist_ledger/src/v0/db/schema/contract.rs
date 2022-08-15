use crate::LedgerError;
use crate::{cfs, LedgerDBSchema};
use sak_contract_std::Storage;
use sak_kv_db::WriteBatch;
use sak_types::CtrAddr;

impl LedgerDBSchema {
    pub(crate) async fn get_ctr_data_by_ctr_addr(
        &self,
        ctr_addr: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let tx_hash = self
            .get_tx_hash_by_ctr_addr(ctr_addr)?
            .ok_or("ctr data does not exist")?;

        let ctr_data = self.get_data(&tx_hash)?.ok_or("data does not exist")?;

        Ok(Some(ctr_data))
    }

    // pub(crate) fn get_ctr_state(
    //     &self,
    //     ctr_addr: &String,
    // ) -> Result<Option<Storage>, LedgerError> {
    //     // let db = &self.kv_db.db_instance;

    //     let ctr_state = self
    //         .get_ctr_state(&ctr_addr)?
    //         .ok_or("ctr state does not exist")?;

    //     let storage: Storage = serde_json::from_slice(&ctr_state)?;

    //     Ok(Some(storage))
    // }

    pub(crate) fn get_tx_hash_by_ctr_addr(
        &self,
        // db: &DB,
        key: &CtrAddr,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_CTR_ADDR)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_ctr_state(
        &self,
        // db: &DB,
        ctr_addr: &CtrAddr,
    ) -> Result<Option<Storage>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CTR_STATE)?;

        match self.db.get_cf(&cf, ctr_addr)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }
}

// writer
impl LedgerDBSchema {
    pub(crate) fn batch_put_ctr_state(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        ctr_addr: &CtrAddr,
        ctr_state: &Storage,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CTR_STATE)?;

        batch.put_cf(&cf, ctr_addr, ctr_state);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hash(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &CtrAddr,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_CTR_ADDR)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
