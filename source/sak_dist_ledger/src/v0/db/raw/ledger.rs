use crate::{cfs, LedgerDB};
use crate::{LedgerError, MerkleNodeLoc};
use sak_kv_db::{IteratorMode, WriteBatch};

impl LedgerDB {
    pub(crate) fn get_latest_cm_idx(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX_CM)?;

        let mut iter = self.db.iterator_cf(&cf, IteratorMode::End);

        match iter.next() {
            Some((cm_idx, cm)) => {
                let val = type_extension::convert_u8_slice_into_u128(&cm_idx)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn batch_put_merkle_node(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        merkle_node_loc: &MerkleNodeLoc,
        node_val: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::MERKLE_NODE)?;

        batch.put_cf(&cf, merkle_node_loc, node_val);

        Ok(())
    }
}
