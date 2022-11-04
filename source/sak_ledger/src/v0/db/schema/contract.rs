use crate::LedgerDB;
use crate::{LedgerCols, LedgerError};
use sak_types::TxHash;

impl LedgerDB {
    pub async fn get_ctr_data_by_ctr_addr(
        &self,
        ctr_addr: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let tx_hash: TxHash = self
            .get_ser(LedgerCols::TxHashByCtrAddr, ctr_addr.as_bytes())?
            .ok_or("TxHashByCtrAddr should exist")?;

        let ctr_data = self
            .get_ser(LedgerCols::Data, tx_hash.as_bytes())?
            .ok_or("TxHashByCtrAddr should exist")?;

        Ok(Some(ctr_data))
    }
}
