use crate::{cfs, LedgerDB};
use crate::{LedgerError, MerkleNodeLoc};
use sak_crypto::{Bls12, Hasher, Proof, ScalarExt};
use sak_kv_db::DB;
use sak_kv_db::{IteratorMode, WriteBatch};
use sak_types::{
    Cm, CmIdx, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Sn, Tx,
    TxCtrOp, TxHash, TxHeight, TxType,
};
use type_extension::U8Arr32;

impl LedgerDB {
    // pub(crate) fn batch_put_ledger_cm_count(
    //     &self,
    //     // db: &DB,
    //     batch: &mut WriteBatch,
    //     cm_count: u128,
    // ) -> Result<(), LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::LEDGER_CM_COUNT)?;

    //     let v = cm_count.to_be_bytes();

    //     batch.put_cf(&cf, keys::SINGLETON, &v);

    //     Ok(())
    // }

    pub(crate) fn get_latest_cm_idx(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

        let mut iter = self.db.iterator_cf(&cf, IteratorMode::End);

        match iter.next() {
            Some((cm, cm_idx)) => {
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
