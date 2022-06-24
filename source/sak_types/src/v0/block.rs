use super::Hashable;
use sak_crypto::sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    validator_sig: String,
    tx_hashes: Vec<String>,
    witness_sigs: Vec<String>,
    created_at: String,
    height: String,
    block_hash: String,
}

impl Block {
    pub fn new(
        validator_sig: String,
        tx_hashes: Vec<String>,
        witness_sigs: Vec<String>,
        created_at: String,
        height: String,
    ) -> Block {
        let block_hash = {
            let mut to_hash = vec![];
            let to_hash = {
                to_hash.push(created_at.as_bytes());

                for tx in tx_hashes.iter() {
                    to_hash.push(tx.as_bytes());
                }

                for sig in witness_sigs.iter() {
                    to_hash.push(sig.as_bytes());
                }

                to_hash.push(created_at.as_bytes());
                to_hash.push(height.as_bytes());

                to_hash.as_slice()
            };

            sak_crypto::compute_hash(to_hash)
        };

        Block {
            validator_sig,
            tx_hashes,
            witness_sigs,
            created_at,
            height,
            block_hash,
        }
    }

    pub fn get_validator_sig(&self) -> &String {
        &self.validator_sig
    }

    pub fn get_witness_sigs(&self) -> &Vec<String> {
        &self.witness_sigs
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_tx_hashes(&self) -> &Vec<String> {
        &self.tx_hashes
    }

    pub fn get_height(&self) -> &String {
        &self.height
    }

    pub fn get_hash(&self) -> &String {
        &self.block_hash
    }
}

// impl std::fmt::Display for Block {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Block (height: {}, tx_hashes: {}, )")
//     }
// }
