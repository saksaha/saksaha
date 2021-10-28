use super::address::Address;
use log::{debug, error, info, warn};
use rand::prelude::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard, OwnedMutexGuard,
};

const CAPACITY: usize = 32;

type Nodes = HashMap<String, Arc<Mutex<TableNode>>>;

pub struct Table {
    map: Mutex<Nodes>,
    indices: Mutex<Vec<String>>,
    rng: Mutex<StdRng>,
    node_tx: Sender<Arc<Mutex<TableNode>>>,
    node_rx: Mutex<Receiver<Arc<Mutex<TableNode>>>>,
}

impl Table {
    pub fn new() -> Table {
        let (node_tx, node_rx) = mpsc::channel::<Arc<Mutex<TableNode>>>(32);

        let map = HashMap::with_capacity(CAPACITY);
        let indices = Vec::new();
        let rng = SeedableRng::from_entropy();

        Table {
            map: Mutex::new(map),
            indices: Mutex::new(indices),
            rng: Mutex::new(rng),
            node_tx,
            node_rx: Mutex::new(node_rx),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        for _ in 0..CAPACITY {
            let empty_node = Arc::new(Mutex::new(TableNode::new_empty()));
            match self.node_tx.send(empty_node).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!(
                        "Can't send empty TableNode to the pool"
                    ));
                }
            }
        }

        Ok(())
    }

    // pub async fn _insert(&self, addr: Address) {
    //     let mut map = self.map.lock().await;
    //     let mut indices = self.indices.lock().await;

    //     let endpoint = addr.endpoint();
    //     let node = TableNode::new(addr);

    //     map.insert(endpoint.clone(), Arc::new(Mutex::new(node)));
    //     indices.push(endpoint);
    // }

    pub async fn register(
        &self,
        table_node: Arc<Mutex<TableNode>>,
        addr: Address,
    ) -> Result<(), String> {
        let mut map = self.map.lock().await;
        let mut indices = self.indices.lock().await;
        let endpoint = addr.endpoint();

        map.insert(endpoint.clone(), table_node);
        indices.push(endpoint);

        Ok(())
    }

    pub async fn reserve(&self) -> Result<Arc<Mutex<TableNode>>, String> {
        let mut node_rx = self.node_rx.lock().await;
        match node_rx.recv().await {
            Some(n) => return Ok(n),
            None => return Err(format!("Can't retrieve tableNode from pool")),
        };
    }

    pub async fn try_reserve(&self) -> Result<Arc<Mutex<TableNode>>, String> {
        let mut node_rx = self.node_rx.lock().await;
        match node_rx.try_recv() {
            Ok(n) => Ok(n),
            Err(err) => Err(format!(
                "Can't reserve a tableNode. Table might be busy, err: {}",
                err
            )),
        }
    }

    pub async fn next(&self) -> Option<OwnedMutexGuard<TableNode>> {
        let map = self.map.lock().await;
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

            let node = match map.get(key) {
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
    pub addr: Option<Address>,
}

impl TableNode {
    pub fn new(addr: Address) -> TableNode {
        TableNode { addr: Some(addr) }
    }

    pub fn new_empty() -> TableNode {
        TableNode { addr: None }
    }
}
