use crate::Block;
use crate::Tx;
use crate::TxCandidate;

#[derive(Debug)]
pub struct BlockCandidate {
    pub validator_sig: String,
    pub tx_candidates: Vec<TxCandidate>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
    // pub block_height: u128,
    // pub merkle_root: String,
}

impl BlockCandidate {
    //
    // pub fn resolve_txs(
    //     self,
    //     // merkle_root: String,
    //     // latest_block_height: u128,
    //     latest_tx_height: u128,
    // ) -> Vec<Tx> {
    //     let txs = {
    //         // let mut tx_hashes = vec![];
    //         let mut txs = vec![];

    //         let mut next_tx_height = latest_tx_height + 1;
    //         for tc in self.tx_candidates {
    //             let tx = tc.upgrade(next_tx_height);
    //             // tx_hashes.push(tx.get_hash().clone());
    //             txs.push(tx);

    //             next_tx_height += 1;
    //         }

    //         (tx_hashes, txs)
    //     };

    //     // let merkle_root;

    //     // let next_block_height = latest_block_height + 1;
    //     // let block = Block::new(
    //     //     self.validator_sig.clone(),
    //     //     tx_hashes,
    //     //     vec![],
    //     //     self.created_at.clone(),
    //     //     next_block_height,
    //     //     merkle_root,
    //     // );

    //     return txs;
    // }
}
