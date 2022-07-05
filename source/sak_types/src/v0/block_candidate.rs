use crate::Block;
use crate::Tx;

#[derive(Debug)]
pub struct BlockCandidate {
    pub validator_sig: String,
    pub transactions: Vec<Tx>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
    pub block_height: u128,
    pub merkle_root: String,
}

impl BlockCandidate {
    pub fn extract(self) -> (Block, Vec<Tx>) {
        let (tx_hashes, txs) = {
            let mut hashes = vec![];
            let mut txs = vec![];

            for tx in self.transactions {
                hashes.push(tx.get_hash().clone());
                txs.push(tx);
            }

            (hashes, txs)
        };

        let block = Block::new(
            self.validator_sig.clone(),
            tx_hashes,
            vec![],
            self.created_at.clone(),
            self.block_height.clone(),
            self.merkle_root.clone(),
        );

        return (block, txs);
    }
}
