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
        self,
        next_block_height: u128,
        next_tx_height: u128,
        next_merkle_rt: [u8; 32],
    ) -> (Block, Vec<Tx>) {
        let mut txs: Vec<Tx> = Vec::new();
        let mut tx_hashes: Vec<String> = vec![];

        for (i, tc) in self.tx_candidates.into_iter().enumerate() {
            let tx = tc.upgrade(next_tx_height + i as u128);
            let tx_hash = tx.get_tx_hash();

            tx_hashes.push(tx_hash.to_owned());
            txs.push(tx);
        }

        let block = Block::new(
            self.validator_sig.clone(),
            tx_hashes,
            self.witness_sigs.clone(),
            self.created_at.clone(),
            next_block_height,
            next_merkle_rt,
        );

        (block, txs)
    }
}
