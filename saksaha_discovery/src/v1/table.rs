use crate::DiscoveryError;
use log::{debug, error, info, warn};
use rand::prelude::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc, Mutex, MutexGuard, OwnedMutexGuard, RwLock, RwLockWriteGuard,
    TryLockError,
};

use super::address::Address;

type Nodes = HashMap<String, Arc<Mutex<TableNode>>>;

pub struct Table {
    nodes: Mutex<Nodes>,
    indices: Mutex<Vec<String>>,
    rng: Mutex<StdRng>,
}

impl Table {
    pub fn new() -> Table {
        let nodes = HashMap::new();
        let indices = Vec::new();
        let rng = SeedableRng::from_entropy();

        Table {
            nodes: Mutex::new(nodes),
            indices: Mutex::new(indices),
            rng: Mutex::new(rng),
        }
    }

    // pub fn nodes(&self) -> Result<MutexGuard<'_, Nodes>, TryLockError> {
    //     self.nodes.try_lock()
    // }

    pub async fn insert(&self, addr: Address) {
        let mut nodes = self.nodes.lock().await;
        let mut indices = self.indices.lock().await;

        let endpoint = addr.endpoint();
        let node = TableNode {
            addr,
        };

        nodes.insert(endpoint.clone(), Arc::new(Mutex::new(node)));
        indices.push(endpoint);
    }

    pub async fn next(&self) -> Option<OwnedMutexGuard<TableNode>> {
        let nodes = self.nodes.lock().await;
        let indices = self.indices.lock().await;
        let mut rng = self.rng.lock().await;
        let seed: usize = rng.gen();

        for i in 0..3 {
            let idx = (seed + i) % indices.len();
            let key = match indices.get(idx) {
                Some(k) => k,
                None => {
                    error!("Table key of idx: {}, not found", idx);
                    continue;
                }
            };

            let node = match nodes.get(key) {
                Some(n) => n.clone(),
                None => {
                    error!(
                        "None TableNode, something might be wrong, idx: {}",
                        idx,
                    );
                    return None;
                }
            };

            let node = match node.try_lock_owned() {
                Ok(n) => n,
                Err(_) => continue,
            };

            return Some(node);
        }

        None
    }
}

#[derive(Debug)]
pub struct TableNode {
    pub addr: Address,
}
