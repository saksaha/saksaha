use crate::{CryptoError, Scalar};

#[derive(Debug)]
pub struct MerkleTree {
    height: u32,
}

#[derive(Debug, Clone)]
pub struct MerklePath {
    pub idx: u128,
    pub direction: bool,
    pub idx_label: String,
}

impl MerkleTree {
    pub fn new(height: u32) -> MerkleTree {
        let t = MerkleTree { height };

        t
    }

    // Calculates which indices and directions are needed to calculate the root
    // of the tree
    pub fn generate_auth_paths(&self, idx: u128) -> Vec<MerklePath> {
        let height = self.height;
        let mut auth_path = vec![];
        let mut curr_idx = idx;

        for h in 0..height {
            let sibling_idx = Self::get_sibling_idx(curr_idx);

            let sibling_dir = if sibling_idx % 2 == 0 { true } else { false };

            let p = MerklePath {
                idx: sibling_idx,
                direction: sibling_dir,
                idx_label: format!("{}_{}", h, sibling_idx),
            };

            auth_path.push(p);

            let parent_idx = Self::get_parent_idx(curr_idx);
            curr_idx = parent_idx;
        }

        auth_path
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
}
