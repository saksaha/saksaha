use crate::hasher::MiMC;
use crate::CryptoError;
use crate::MerkleTree;
use crate::Scalar;
use std::collections::HashMap;

// {height}_{idx}. Index starts from 0.
// e.g. "0_1" is the second element in the leaf (bottom_most) height.
type TreeIdx = String;

// Merkle tree simulator
pub struct MerkleTreeSim {
    height: u32,
    pub nodes: HashMap<TreeIdx, Scalar>,
    leaf_count: u32,
    pub merkle_tree: MerkleTree,
}

impl MerkleTreeSim {
    pub fn init(height: u32, leaves: Vec<Scalar>) -> Result<MerkleTreeSim, CryptoError> {
        let mut mk_tree_init = MerkleTreeSim {
            height,
            nodes: HashMap::new(),
            leaf_count: 0,
            merkle_tree: MerkleTree::new(height),
        };

        for (leaf_idx, leaf) in leaves.iter().enumerate() {
            mk_tree_init.update_root(leaf_idx as u32, *leaf)?;
        }

        Ok(mk_tree_init)
    }

    pub fn get_leaf_count(&self) -> u32 {
        self.leaf_count
    }

    pub fn get_merkle_rt(&self) -> Scalar {
        match self.nodes.get(format!("{}_0", self.height).as_str()) {
            Some(t) => *t,
            None => Scalar::default(),
        }
    }

    pub fn add_leaf_node(&mut self, cm: Scalar) {
        let idx = format!("{}_{}", 0, self.leaf_count);

        self.leaf_count += 1;

        self.nodes.insert(idx, cm);
    }

    pub fn update_root(&mut self, leaf_idx: u32, leaf: Scalar) -> Result<(), CryptoError> {
        let hasher = MiMC::new();
        let auth_path = self.merkle_tree.generate_auth_paths(leaf_idx as u128);

        self.add_leaf_node(leaf);

        for (height, path) in auth_path.iter().enumerate() {
            let sibling_idx = path.idx;

            let sibling_loc;
            let sibling_node: Scalar;

            let curr_idx = match path.direction {
                true => {
                    let ci = sibling_idx + 1;
                    sibling_loc = format!("{}_{}", height, sibling_idx);
                    sibling_node = match self.nodes.get(&sibling_loc) {
                        Some(t) => *t,
                        None => Scalar::default(),
                    };
                    ci
                }
                false => {
                    let ci = sibling_idx - 1;
                    sibling_node = Scalar::default();
                    ci
                }
            };

            let curr_loc = format!("{}_{}", height, curr_idx);
            let curr_node = match self.nodes.get(&curr_loc) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            let lv;
            let rv;

            if path.direction {
                lv = sibling_node;
                rv = curr_node;
            } else {
                lv = curr_node;
                rv = sibling_node;
            }

            let merkle_node = hasher.mimc_scalar(lv, rv);

            let parent_idx = MerkleTree::get_parent_idx(curr_idx);
            let update_loc = format!("{}_{}", height + 1, parent_idx);

            self.nodes.insert(update_loc, merkle_node);
        }
        Ok(())
    }
}
