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
        next_merkle_rt: Vec<u8>,
    ) -> (Block, Vec<Tx>) {
        let block_height = match latest_block_height {
            Some(h) => h,
            None => 0,
        };

        let tx_height = match latest_tx_height {
            Some(h) => h,
            None => 0,
        };

        let mut txs: Vec<Tx> = Vec::new();
        let mut tx_hashes: Vec<String> = vec![];

        for (i, tc) in self.tx_candidates.into_iter().enumerate() {
            let tx = tc.upgrade(tx_height + i as u128);

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
            next_merkle_rt, //
        );

        (block, txs)
    }
}
