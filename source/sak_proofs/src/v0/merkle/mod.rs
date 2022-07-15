use crate::ProofError;
use log::debug;
use sak_crypto::{mimc, Scalar};
use std::convert::TryInto;

pub const TREE_DEPTH: u32 = 5;

pub const TREE_CAPACITY: u32 = 2_u32.pow(TREE_DEPTH);

#[derive(Debug)]
pub struct MerkleTree {
    // pub nodes: Vec<Vec<Node>>,
    // pub height: usize,
    // pub data: Vec<u32>,
    pub leaves: Vec<[u8; 32]>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub val: Option<[u32; 1]>,
    pub hash: Scalar,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub direction: bool,
    pub hash: Scalar,
}

impl MerkleTree {
    pub fn init(
        data: &[[u8; 32]],
        height: usize,
        constants: &[Scalar],
        hasher: &dyn Fn(u64, u64) -> Scalar,
    ) -> Result<MerkleTree, ProofError> {
        let data_count = data.len();

        if data_count as u32 > TREE_CAPACITY {
            return Err(format!(
                "Data too many to be contained in the tree, \
                len: {}, capacity: {}",
                data.len(),
                TREE_CAPACITY
            )
            .into());
        }

        let mut leaves = vec![[0u8; 32]; TREE_CAPACITY.try_into()?];

        leaves.copy_from_slice(data);

        // println!(
        //     "Create tree, leaf_count: {}, height: {}, data: {:?}",
        //     leaf_count, height, data,
        // );

        // for h in 1..=height {
        //     let child_nodes = nodes.get_mut((h - 1) as usize).unwrap();
        //     let mut nodes_at_height = vec![];

        //     if child_nodes.len() % 2 == 1 {
        //         let l = child_nodes.last().unwrap();
        //         let last_node = copy_node(l);
        //         child_nodes.push(last_node);
        //     }

        //     let mut xl = Scalar::default();

        //     for (idx, cn) in child_nodes.iter().enumerate() {
        //         if idx % 2 == 0 {
        //             xl = cn.hash;
        //         } else {
        //             let xr = cn.hash;
        //             let hs = mimc::mimc(xl, xr, &constants);

        //             let n = Node {
        //                 val: None,
        //                 hash: hs,
        //             };

        //             nodes_at_height.push(n);
        //         }
        //     }
        //     nodes.push(nodes_at_height);
        // }

        let t = MerkleTree {
            // nodes,
            // height,
            // data: d,
            leaves,
        };

        Ok(t)
    }

    // pub fn get_root(&self) -> &Node {
    //     let highest_nodes = self.nodes.get(self.nodes.len() - 1).unwrap();
    //     highest_nodes.get(0).unwrap()
    // }

    // pub fn compute_root(&self) -> &[u8; 32] {
    //     &[0u8; 32]
    //     // let highest_nodes = self.nodes.get(self.nodes.len() - 1).unwrap();
    //     // highest_nodes.get(0).unwrap()
    // }

    // pub fn sibling(&self, height: u64, idx: u64) -> &Node {
    //     let len = self.nodes.len() as u64;
    //     if idx >= len - 1 {
    //         panic!("Invalid idx, cannot get sibling node");
    //     }

    //     let sibling_idx = get_sibling_idx(idx);
    //     let nodes_at_height = self.nodes.get(height as usize).unwrap();
    //     let n = nodes_at_height.get(sibling_idx as usize).unwrap();

    //     n
    // }

    pub fn generate_auth_paths(&self, idx: u64) -> Vec<Path> {
        let height = self.height;
        let mut auth_path = vec![];

        let mut curr_idx = idx;

        for h in 0..height {
            let sibling_idx = get_sibling_idx(curr_idx);

            let sibling = self
                .nodes
                .get(h as usize)
                .unwrap()
                .get(sibling_idx as usize)
                .unwrap();

            let direction = if sibling_idx % 2 == 0 { true } else { false };

            let p = Path {
                direction,
                hash: sibling.hash.clone(),
            };

            auth_path.push(p);

            let parent_idx = get_parent_idx(curr_idx);
            curr_idx = parent_idx;
        }

        auth_path
    }

    pub fn display_tree(&self) {
        for (idx, l) in self.leaves.iter().enumerate() {
            debug!("leaf idx: {}: leaf: {:?}\n", idx, l);
        }
    }
}

pub fn get_auth_path(leaf_idx: u64) -> Vec<u64> {
    let mut auth_path = vec![];

    let mut curr_idx = leaf_idx;

    for _curr_height in 0..TREE_DEPTH {
        let sibling_idx = get_sibling_idx(curr_idx);

        let location = sibling_idx;

        auth_path.push(location);

        let parent_idx = get_parent_idx(curr_idx);
        curr_idx = parent_idx;
    }

    auth_path
}

fn copy_node(node: &Node) -> Node {
    Node {
        val: node.val,

        hash: node.hash.clone(),
    }
}

fn get_sibling_idx(idx: u64) -> u64 {
    if idx % 2 == 0 {
        idx + 1
    } else {
        idx - 1
    }
}

pub fn get_parent_idx(idx: u64) -> u64 {
    idx / 2
}
