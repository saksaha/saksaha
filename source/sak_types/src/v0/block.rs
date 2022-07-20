use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Block {
    pub validator_sig: String,
    pub tx_hashes: Vec<String>,
    pub witness_sigs: Vec<String>,
    pub created_at: String,
    pub block_height: u128,
    pub merkle_rt: [u8; 32],
    pub total_cm_count: u128,
    block_hash: String,
}

impl Block {
    pub fn new(
        validator_sig: String,
        tx_hashes: Vec<String>,
        witness_sigs: Vec<String>,
        created_at: String,
        block_height: u128,
        merkle_rt: [u8; 32],
        total_cm_count: u128,
    ) -> Block {
        let block_hash = {
            let mut to_hash = vec![];
            let to_hash = {
                to_hash.push(validator_sig.as_bytes());

                for tx in tx_hashes.iter() {
                    to_hash.push(tx.as_bytes());
                }

                for sig in witness_sigs.iter() {
                    to_hash.push(sig.as_bytes());
                }

                to_hash.push(created_at.as_bytes());

                to_hash.push(merkle_rt.as_slice());

                to_hash.as_slice()
            };

            sak_crypto::compute_hash(to_hash)
        };

        Block {
            validator_sig,
            tx_hashes,
            witness_sigs,
            created_at,
            block_height,
            merkle_rt,
            block_hash,
            total_cm_count,
        }
    }

    pub fn get_block_hash(&self) -> &String {
        &self.block_hash
    }
}
