use log::debug;
use sak_crypto::{mimc, Scalar};

pub const TREE_DEPTH: u32 = 5;
pub const TREE_CAPACITY: u32 = 2_u32.pow(TREE_DEPTH);

#[derive(Debug)]
pub struct MerkleTree {
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub val: Option<[u32; 1]>,
    pub hash: Scalar,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub idx: u128,
    pub direction: bool,
}

impl MerkleTree {
    pub fn new(height: u32) -> MerkleTree {
        let t = MerkleTree { height };

        t
    }

    pub fn generate_auth_paths(&self, idx: u128) -> Vec<Path> {
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

fn get_sibling_idx(idx: u128) -> u128 {
    if idx % 2 == 0 {
        idx + 1
    } else {
        idx - 1
    }
}

pub fn get_parent_idx(idx: u128) -> u128 {
    idx / 2
}
