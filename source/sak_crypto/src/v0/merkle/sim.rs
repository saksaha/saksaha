use crate::hasher::MiMC;
use crate::CryptoError;
use crate::MerkleTree;
use crate::Scalar;
use crate::ScalarExt;
use std::collections::HashMap;
use std::convert::TryInto;
use type_extension::U8Array;

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
    pub fn init(height: u32, leaves: Vec<Scalar>) -> MerkleTreeSim {
        let mut mk_tree_init = MerkleTreeSim {
            height,
            nodes: HashMap::new(),
            leaf_count: 0,
            merkle_tree: MerkleTree::new(height),
        };
        println!("init all nodes start");
        mk_tree_init.init_all_nodes(leaves.len() as u32);
        println!("init all nodes finish");

        for (leaf_idx, leaf) in leaves.iter().enumerate() {
            mk_tree_init.update_root(leaf_idx.try_into().unwrap(), *leaf);
        }

        mk_tree_init
    }

    pub fn init_all_nodes(&mut self, cm_len: u32) {
        for i in 1..self.height {
            let zero_node = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
            let base: u32 = 2;
            let nodes_len = (cm_len as f32 / (base.pow(i) as f32)).floor() as u32;
            println!("nodes_len: {}", nodes_len);
            for j in 1..1 + nodes_len {
                let tmp_loc = format!("{}_{}", i, j);
                self.nodes.insert(tmp_loc, zero_node);
            }
        }
    }

    pub fn get_leaf_count(&self) -> u32 {
        self.leaf_count
    }

    pub fn get_merkle_rt(&self) -> Scalar {
        *self
            .nodes
            .get(format!("{}_0", self.height).as_str())
            .unwrap()
    }

    pub fn add_leaf_node(&mut self, cm: Scalar) {
        let idx = format!("{}_{}", 0, self.leaf_count);

        self.leaf_count += 1;

        self.nodes.insert(idx, cm);
    }

    pub fn update_root(&mut self, leaf_idx: u32, leaf: Scalar) {
        let hahser = MiMC::new();
        let empty_node = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
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
                    sibling_node = *self.nodes.get(&sibling_loc).unwrap();
                    ci
                }
                false => {
                    let ci = sibling_idx - 1;
                    sibling_node = empty_node;
                    ci
                }
            };

            let curr_loc = format!("{}_{}", height, curr_idx);
            let curr_node = *self.nodes.get(&curr_loc).unwrap();

            let lv;
            let rv;

            if path.direction {
                lv = sibling_node;
                rv = curr_node;
            } else {
                lv = curr_node;
                rv = sibling_node;
            }

            let merkle_node = hahser.mimc_scalar(lv, rv);

            let parent_idx = MerkleTree::get_parent_idx(curr_idx);
            let update_loc = format!("{}_{}", height + 1, parent_idx);

            self.nodes.insert(update_loc, merkle_node);
        }
    }
}
