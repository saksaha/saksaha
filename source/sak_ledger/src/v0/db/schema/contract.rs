use crate::LedgerDB;
use crate::{LedgerCols, LedgerError};
use sak_types::TxHash;

impl LedgerDB {
    pub async fn get_ctr_data_by_ctr_addr(
        &self,
        ctr_addr: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let tx_hash: TxHash = self
            .get(LedgerCols::TxHashByCtrAddr, ctr_addr.as_bytes())?
            .ok_or("TxHashByCtrAddr should exist")?;

        let ctr_data = self
            .get(LedgerCols::Data, tx_hash.as_bytes())?
            .ok_or("Data should exist")?;

        Ok(Some(ctr_data))
    }
}
