use log::debug;
use sak_crypto::{mimc, Hasher, Scalar, ScalarExt};

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

// pub fn get_merkle_node_value(pos: String) -> Scalar {
//     let hasher = Hasher::new();

//     let mut a = pos.split("_");

//     let mut height = a.next().unwrap().to_owned().parse::<u128>().unwrap();
//     let idx = a.next().unwrap().to_owned().parse::<u128>().unwrap();

//     // get all the base node
//     let mut begin: u128 = idx;
//     let mut end: u128 = idx;

//     let mut cm_idx = Vec::new();
//     while height != 0 {
//         height -= 1;

//         begin = begin * 2;
//         end = end * 2 + 1;
//     }

//     for idx in begin..=end {
//         cm_idx.push(idx);
//     }
//     println!("cm_idx : {:#?}", cm_idx);

//     // climb up

//     let mut tmp = Vec::new();
//     while tmp.len() != 1 {
//         for idx in (0..cm_idx.len()).step_by(2) {
//             hasher.mimc_scalar(
//                 Scalar::from_raw(cm_idx[idx]),
//                 Scalar::from_raw(cm_idx[idx + 1]),
//             );
//         }
//     }

//     Scalar::default()
// }
