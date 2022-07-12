use serde::{Deserialize, Serialize};

use crate::Tx;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    validator_sig: String,

    tx_hashes: Vec<String>,
    witness_sigs: Vec<String>,
    created_at: String,
    block_height: u128,
    merkle_rt: Vec<u8>,
    block_hash: String,

    //
    #[serde(skip_serializing, skip_deserializing)]
    transactions: Vec<Tx>,
}

impl Block {
    pub fn new(
        validator_sig: String,
        tx_hashes: Vec<String>,
        witness_sigs: Vec<String>,
        created_at: String,
        block_height: u128,
        merkle_rt: Vec<u8>,
        transactions: Vec<Tx>,
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
            transactions,
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

    pub fn get_height(&self) -> &u128 {
        &self.block_height
    }

    pub fn get_hash(&self) -> &String {
        &self.block_hash
    }

    pub fn get_merkle_rt(&self) -> &Vec<u8> {
        &self.merkle_rt
    }
}
