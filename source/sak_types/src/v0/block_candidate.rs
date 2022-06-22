use crate::Block;
use crate::Hashable;
use crate::Transaction;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct BlockCandidate {
    pub validator_sig: String,
    pub transactions: Vec<Transaction>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
    pub height: String,
}

impl BlockCandidate {
    pub fn extract(self) -> (Block, Vec<Transaction>) {
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
            self.height.clone(),
        );

        return (block, txs);
    }
}
