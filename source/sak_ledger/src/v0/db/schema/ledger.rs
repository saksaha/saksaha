use crate::LedgerError;
use crate::{LedgerCols, LedgerDB};
use sak_crypto::ScalarExt;
use type_extension::U8Array;

impl LedgerDB {
    pub fn get_latest_cm_idx(&self) -> Result<Option<u128>, LedgerError> {
        let mut iter = self.iter(LedgerCols::CMByCMIdx)?;

        match iter.next() {
            Some((cm_idx, _cm)) => {
                let val = type_extension::convert_u8_slice_into_u128(&cm_idx)?;

                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    pub(crate) fn get_merkle_node(&self, key: &String) -> Result<[u8; 32], LedgerError> {
        match self.get_ser(LedgerCols::MerkleNode, key.as_bytes())? {
            Some(v) => Ok(v),
            None => {
                let zero_value = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                Ok(zero_value.to_bytes())
            }
        }
    }

    pub fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerError> {
        let mut iter = self.iter(LedgerCols::BlockHash)?;

        let (height_bytes, _hash) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let height = type_extension::convert_u8_slice_into_u128(&height_bytes)?;

        Ok(Some(height))
    }
}
