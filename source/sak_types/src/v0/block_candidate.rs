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
    pub fn upgrade(
        &self,
        latest_block_height: Option<u128>,
        latest_tx_height: Option<u128>,
        latest_merkle_root: Option<Vec<u8>>,
    ) -> (Block, Vec<Tx>) {
        let block_height = match latest_block_height {
            Some(h) => h,
            None => 0,
        };

        let tx_height = match latest_tx_height {
            Some(h) => h,
            None => 0,
        };

        let mut merkle_root = match latest_merkle_root {
            Some(rt) => rt,
            None => vec![],
        };

        let mut txs: Vec<Tx> = Vec::new();
        let mut tx_hashes: Vec<String> = vec![];

        for (i, tc) in self.tx_candidates.iter().enumerate() {
            let tx = tc.clone().upgrade(tx_height + i as u128);

            // merkle_root = tx.get_merkle_rt().clone();

            txs.push(tx.clone());
            let tx_hash = tx.get_tx_hash();
            tx_hashes.push(tx_hash.clone());
        }

        let block = Block::new(
            self.validator_sig.clone(),
            tx_hashes,
            self.witness_sigs.clone(),
            self.created_at.clone(),
            block_height,
            merkle_root, //
        );

        (block, txs)
    }

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
