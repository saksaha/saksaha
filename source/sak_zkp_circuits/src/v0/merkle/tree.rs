use log::debug;
use sak_crypto::{Scalar, ScalarExt};

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
pub struct MerklePath {
    pub idx: u128,
    pub direction: bool,
}

impl MerkleTree {
    pub fn new(height: u32) -> MerkleTree {
        let t = MerkleTree { height };

        t
    }

    // Calculates which indices and directions are needed to calculate merkle
    pub fn generate_auth_paths(&self, idx: u128) -> Vec<MerklePath> {
        let height = self.height;
        let mut auth_path = vec![];
        let mut curr_idx = idx;

        for _ in 0..height {
            let sibling_idx = get_sibling_idx(curr_idx);

            let direction = if sibling_idx % 2 == 0 { true } else { false };

            let p = MerklePath {
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
