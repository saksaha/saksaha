use crate::CryptoError;
use crate::MerkleTree;
use crate::Scalar;
use crate::ScalarExt;
use std::collections::HashMap;
use type_extension::U8Array;

// {height}_{idx}. Index starts from 0.
// e.g. "0_1" is the second element in the leaf (bottom_most) height.
type TreeIdx = String;

// Merkle tree simulator
pub struct MerkleTreeSim {
    height: u32,
    nodes: HashMap<TreeIdx, Scalar>,
    leaf_count: u32,
    merkle_tree: MerkleTree,
}

impl MerkleTreeSim {
    pub fn init(height: u32, leaves: Vec<Scalar>) -> MerkleTreeSim {
        MerkleTreeSim {
            height,
            nodes: HashMap::new(),
            leaf_count: 0,
            merkle_tree: MerkleTree::new(height),
        }
    }

    pub fn get_leaf_count(&self) -> u32 {
        self.leaf_count
    }

    pub fn add_leaf_node(&mut self, cm: Scalar) {
        self.leaf_count += 1;

        let idx = format!("{}_{}", 0, self.leaf_count);

        self.nodes.insert(idx, cm);

        self.update_root(self.leaf_count);
    }

    pub fn update_root(&mut self, leaf_idx: u32) {
        let auth_path = self.merkle_tree.generate_auth_paths(leaf_idx as u128);

        println!("auth_path: {:?}", auth_path);

        // for (h, path) in merkle_paths.iter().enumerate() {
        //     println!("h: {}, path: {:?}", h, path);

        //     let node = self.nodes.get(&path.idx_label).unwrap_or(
        //         &ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap(),
        //     );

        //     if path.direction {

        //     } else {
        //     }
        // }

        for (height, path) in auth_path.iter().enumerate() {
            // let curr_idx = path.idx;
            // let sibling_idx = match path.direction {
            //     true => path.idx + 1,
            //     false => path.idx - 1,
            // };

            // let sibling_loc = format!("{}_{}", height, sibling_idx);
            // let sibling_node = match merkle_update.get(&sibling_loc) {
            //     Some(n) => *n,
            //     None => apis.get_merkle_node(&sibling_loc).await?,
            // };

            // let curr_loc = format!("{}_{}", height, curr_idx);
            // let curr_node = match merkle_update.get(&curr_loc) {
            //     Some(n) => *n,
            //     None => apis.get_merkle_node(&curr_loc).await?,
            // };

            // let merkle_node =
            //     apis.hasher.mimc(&curr_node, &sibling_node)?.to_bytes();

            // let parent_idx = MerkleTree::get_parent_idx(curr_idx);
            // let update_loc = format!("{}_{}", height + 1, parent_idx);

            // merkle_update.insert(update_loc, merkle_node);
        }
    }
}
