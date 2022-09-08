use crate::Block;
use crate::CmIdx;
use crate::Tx;
use crate::TxCandidate;

#[derive(Debug)]
pub struct BlockCandidate {
    pub validator_sig: String,
    pub tx_candidates: Vec<TxCandidate>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
}

impl BlockCandidate {
    pub fn upgrade(
        self,
        next_block_height: u128,
        mut next_cm_idx: CmIdx,
        next_merkle_rt: [u8; 32],
    ) -> (Block, Vec<Tx>) {
        let mut txs: Vec<Tx> = Vec::new();
        let mut tx_hashes: Vec<String> = vec![];

        for (idx, tc) in self.tx_candidates.into_iter().enumerate() {
            let tx = tc.upgrade(next_cm_idx);
            next_cm_idx = *tx
                .get_cm_idxes()
                .last()
                .unwrap_or(&(next_cm_idx + idx as u128 + 1))
                + 1;

            let tx_hash = tx.get_tx_hash();

            tx_hashes.push(tx_hash.to_owned());
            txs.push(tx);
        }

        let block = Block::new(
            self.validator_sig.clone(),
            tx_hashes,
            self.witness_sigs.clone(),
            self.created_at,
            next_block_height,
            next_merkle_rt,
        );

        (block, txs)
    }

    // pub fn update_tx_candidates(
    //     &mut self,
    //     valid_tx_candidates: Vec<TxCandidate>,
    // ) {
    //     self.tx_candidates = valid_tx_candidates;
    // }
}
