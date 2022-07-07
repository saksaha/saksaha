use bls12_381::Scalar;

use crate::mimc;

#[derive(Debug)]
pub struct MerkleTree {
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

fn get_parent_idx(idx: u64) -> u64 {
    idx / 2
}

impl MerkleTree {
    pub fn new(
        data: Vec<u32>,
        height: usize,
        constants: &[Scalar],
        hasher: &dyn Fn(u64, u64) -> Scalar,
    ) -> MerkleTree {
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

            let hash = hasher(xl, xr);

            let n = Node {
                val: Some([l]),
                hash,
            };

            leaves.push(n);
        }

        // println!("[*] leaves: {:#?}", leaves);
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

            for (idx, cn) in child_nodes.iter().enumerate() {
                if idx % 2 == 0 {
                    xl = cn.hash;
                } else {
                    let xr = cn.hash;
                    let hs = mimc(xl, xr, &constants);

                    let n = Node {
                        val: None,
                        hash: hs,
                    };

                    nodes_at_height.push(n);
                }
            }
            nodes.push(nodes_at_height);
        }

        // for (idx, e) in nodes.iter().enumerate() {
        // println!(
        //     "node idx: {}: node_len: {}, node: {:?}\n",
        //     idx,
        //     e.len(),
        //     e
        // );
        // }

        MerkleTree {
            nodes,
            height,
            data: d,
        }
    }

    pub fn get_root(&self) -> &Node {
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
