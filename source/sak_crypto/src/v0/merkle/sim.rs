use crate::CryptoError;
use crate::Scalar;
use std::collections::HashMap;

// {height}_{idx}. Index starts from 0.
// e.g. "0_1" is the second element in the leaf (bottom_most) height.
type TreeIdx = String;

// Merkle tree simulator
pub struct MerkleTreeSim {
    height: u32,
    nodes: HashMap<TreeIdx, Scalar>,
    leaf_count: u32,
}

impl MerkleTreeSim {
    pub fn new(height: u32) -> MerkleTreeSim {
        MerkleTreeSim {
            height,
            nodes: HashMap::new(),
            leaf_count: 0,
        }
    }

    pub fn get_leaf_count(&self) -> u32 {
        self.leaf_count
    }

    pub fn add_leaf_node(&mut self, cm: Scalar) {
        self.leaf_count += 1;

        let idx = format!("{}_{}", 0, self.leaf_count);

        self.nodes.insert(idx, cm);

        self.update_root();
    }

    pub fn update_root(&self) {}
}
