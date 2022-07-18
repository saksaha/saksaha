use crate::ProofError;
use log::debug;
use sak_crypto::{mimc, Scalar};
use std::convert::TryInto;

pub const TREE_DEPTH: u32 = 5;
pub const TREE_CAPACITY: u32 = 2_u32.pow(TREE_DEPTH);

#[derive(Debug)]
pub struct MerkleTree {
    // pub nodes: Vec<Vec<Node>>,
    pub height: u32,
    // pub data: Vec<u32>,
    // pub leaves: Vec<[u8; 32]>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub val: Option<[u32; 1]>,
    pub hash: Scalar,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub idx: u32,
    pub direction: bool,
}

impl MerkleTree {
    pub fn new(height: u32, constants: &[Scalar]) -> MerkleTree {
        let t = MerkleTree { height };

        t
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

    pub fn generate_auth_paths(&self, idx: u32) -> Vec<Path> {
        let height = self.height;
        let mut auth_path = vec![];
        let mut curr_idx = idx;

        for _ in 0..height {
            let sibling_idx = get_sibling_idx(curr_idx);

            let direction = if sibling_idx % 2 == 0 { true } else { false };

            let p = Path {
                idx: sibling_idx,
                direction,
            };

            auth_path.push(p);

            let parent_idx = get_parent_idx(curr_idx);
            curr_idx = parent_idx;
        }

        auth_path
    }
}

// pub fn get_auth_path(leaf_idx: u32) -> Vec<u32> {
//     let mut auth_path = vec![];

//     let mut curr_idx = leaf_idx;

//     for _curr_height in 0..TREE_DEPTH {
//         let sibling_idx = get_sibling_idx(curr_idx);

//         let location = sibling_idx;

//         auth_path.push(location);

//         let parent_idx = get_parent_idx(curr_idx);
//         curr_idx = parent_idx;
//     }

//     auth_path
// }

fn get_sibling_idx(idx: u32) -> u32 {
    if idx % 2 == 0 {
        idx + 1
    } else {
        idx - 1
    }
}

pub fn get_parent_idx(idx: u32) -> u32 {
    idx / 2
}
