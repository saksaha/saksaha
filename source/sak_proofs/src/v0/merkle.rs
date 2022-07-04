use bls12_381::Scalar;
use ff::Field;
use ff::PrimeField;
use rand::thread_rng;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Vec<Node>>,
    pub height: usize,
    pub data: Vec<u32>,
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

const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn copy_node(node: &Node) -> Node {
    Node {
        val: node.val,

        hash: node.hash.clone(),
    }
}

fn log_floor(num: usize) -> u32 {
    num_bits::<u64>() as u32 - num.leading_zeros() - 1
}

fn get_sibling_idx(idx: u64) -> u64 {
    if idx % 2 == 0 {
        idx + 1
    } else {
        idx - 1
    }
}

fn get_parent_idx(idx: u64) -> u64 {
    idx / 2
}

pub fn mimc<S: PrimeField>(mut xl: S, mut xr: S, constants: &[S]) -> S {
    // assert_eq!(constants.len(), MIMC_ROUNDS);

    for c in constants {
        let mut tmp1 = xl;
        tmp1.add_assign(c);
        let mut tmp2 = tmp1.square();
        tmp2.mul_assign(&tmp1);
        tmp2.add_assign(&xr);
        xr = xl;
        xl = tmp2;
    }

    xl
}

impl Tree {
    pub fn new(data: Vec<u32>, height: usize, constants: &[Scalar]) -> Tree {
        let mut leaves = vec![];
        let leaf_count = data.len();

        println!(
            "Create tree, leaf_count: {}, height: {}, data: {:?}",
            leaf_count, height, data,
        );

        let d = data.clone();

        for l in data.into_iter() {
            let xl: u64 = l.into();
            let xr: u64 = (l + 1).into();
            let hash = mimc(Scalar::from(xl), Scalar::from(xr), constants);

            let n = Node {
                val: Some([l]),
                hash,
            };

            leaves.push(n);
        }

        let mut nodes = vec![leaves];

        for h in 1..=height {
            let child_nodes = nodes.get_mut((h - 1) as usize).unwrap();
            let mut nodes_at_height = vec![];

            if child_nodes.len() % 2 == 1 {
                let l = child_nodes.last().unwrap();
                let last_node = copy_node(l);
                child_nodes.push(last_node);
            }

            let mut xl = Scalar::default();
            // let mut xr = Scalar::default();

            // let mut combined: [u8; 64] = [0; 64];
            for (idx, cn) in child_nodes.iter().enumerate() {
                if idx % 2 == 0 {
                    // combined[..32].clone_from_slice(&cn.hash);
                    // hasher.update(cn.hash.to_owned());
                    xl = cn.hash;
                } else {
                    // combined[32..].clone_from_slice(&cn.hash);
                    // println!("combining: {:?}", combined);

                    // let hs = Sha256::digest(&combined);
                    let xr = cn.hash;
                    let hs = mimc(xl, xr, &constants);
                    // let mut hash: [u8; 32] = Default::default();
                    // hash.copy_from_slice(&hs[..]);

                    // println!("height: {}, hash: {:?}", h, hs);

                    let n = Node {
                        val: None,
                        hash: hs,
                    };

                    nodes_at_height.push(n);
                    // combined = [0; 64];
                }
            }
            nodes.push(nodes_at_height);
        }

        for (idx, e) in nodes.iter().enumerate() {
            println!(
                "node idx: {}: node_len: {}, node: {:?}\n",
                idx,
                e.len(),
                e
            );
        }

        Tree {
            nodes,
            height,
            data: d,
        }
    }

    pub fn root(&self) -> &Node {
        let highest_nodes = self.nodes.get(self.nodes.len() - 1).unwrap();
        highest_nodes.get(0).unwrap()
    }

    pub fn sibling(&self, height: usize, idx: u64) -> &Node {
        let len: u64 = self.nodes.len() as u64;
        if idx >= len - 1 {
            panic!("Invalid idx, cannot get sibling node");
        }

        let sibling_idx = get_sibling_idx(idx);
        let nodes_at_height = self.nodes.get(height).unwrap();
        let n = nodes_at_height.get(sibling_idx as usize).unwrap();

        n
    }

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
}
