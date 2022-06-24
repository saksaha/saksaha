use crate::Block;
use crate::Hashable;
use crate::Tx;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct BlockCandidate {
    pub validator_sig: String,
    pub transactions: Vec<Tx>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
    pub height: String,
}

impl BlockCandidate {
    pub fn get_hash(&self) -> String {
        let block_hash = {
            let mut to_hash = vec![];
            let to_hash = {
                to_hash.push(self.created_at.as_bytes());

                for tx in self.transactions.iter() {
                    to_hash.push(tx.get_hash().as_bytes());
                }

                for sig in self.witness_sigs.iter() {
                    to_hash.push(sig.as_bytes());
                }

                to_hash.push(self.created_at.as_bytes());
                to_hash.push(self.height.as_bytes());

                to_hash.as_slice()
            };

            sak_crypto::compute_hash(to_hash)
        };

        block_hash
    }

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
            self.height.clone(),
        );

        return (block, txs);
    }
}
