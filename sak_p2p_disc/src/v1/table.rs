use super::address::Address;
use futures::Future;
use log::{debug, error, info, warn};
use rand::prelude::*;
use sak_crypto::Signature;
use sak_p2p_identity::PUBLIC_KEY_LEN;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard, OwnedMutexGuard,
};

const CAPACITY: usize = 32;

type Nodes = HashMap<String, Arc<Mutex<TableNode>>>;

pub struct Table {
    map: Mutex<Nodes>,
    keys: Mutex<Vec<String>>,
    rng: Mutex<StdRng>,
    slots_tx: Sender<Arc<Mutex<TableNode>>>,
    slots_rx: Mutex<Receiver<Arc<Mutex<TableNode>>>>,
}

impl Table {
    pub fn new() -> Table {
        let (slots_tx, slots_rx) =
            mpsc::channel::<Arc<Mutex<TableNode>>>(CAPACITY);

        let map = HashMap::with_capacity(CAPACITY);
        let keys = Vec::new();
        let rng = SeedableRng::from_entropy();

        Table {
            map: Mutex::new(map),
            keys: Mutex::new(keys),
            rng: Mutex::new(rng),
            slots_tx,
            slots_rx: Mutex::new(slots_rx),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        for _ in 0..CAPACITY {
            let empty_node = Arc::new(Mutex::new(TableNode::new_empty()));
            println!("333: {:p}", empty_node);

            match self.slots_tx.send(empty_node).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!(
                        "Can't send empty TableNode to the pool, err: {}",
                        err
                    ));
                }
            }
        }

        Ok(())
    }

    pub async fn find(
        &self,
        endpoint: &String,
    ) -> Option<Arc<Mutex<TableNode>>> {
        let map = self.map.lock().await;
        if let Some(n) = map.get(endpoint) {
            return Some(n.clone());
        } else {
            return None;
        }
    }

    // pub async fn update<F>(&self, updater: F) -> Result<(), String>
    // where
    //     F: Future,
    // {
    //     Ok(())
    // }

    pub async fn update(
        &self,
        table_node: Arc<Mutex<TableNode>>,
    ) -> Result<(), String> {


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
        endpoint: String,
        table_node: Arc<Mutex<TableNode>>,
    ) -> Result<(), String> {
        let mut map = self.map.lock().await;
        let mut keys = self.keys.lock().await;

        map.insert(endpoint.clone(), table_node);
        keys.push(endpoint);

        Ok(())
    }

    pub async fn reserve(&self) -> Result<Arc<Mutex<TableNode>>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.recv().await {
            Some(n) => return Ok(n),
            None => return Err(format!("Can't retrieve tableNode from pool")),
        };
    }

    pub async fn try_reserve(&self) -> Result<Arc<Mutex<TableNode>>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.try_recv() {
            Ok(n) => Ok(n),
            Err(err) => Err(format!(
                "Can't reserve a tableNode. Table might be busy, err: {}",
                err
            )),
        }
    }

    pub async fn next(&self) -> Option<OwnedMutexGuard<TableNode>> {
        let map = self.map.lock().await;
        let keys = self.keys.lock().await;
        let mut rng = self.rng.lock().await;
        let seed: usize = rng.gen();

        for i in 0..3 {
            let idx = (seed + i) % keys.len();
            let key = match keys.get(idx) {
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

pub struct TableNode {
    pub addr: Option<Address>,
    pub record: Option<Record>,
}

impl TableNode {
    pub fn new(addr: Address) -> TableNode {
        TableNode {
            addr: Some(addr),
            record: None,
        }
    }

    pub fn new_empty() -> TableNode {
        TableNode {
            addr: None,
            record: None,
        }
    }
}

pub struct Record {
    pub sig: Signature,
    pub p2p_port: u16,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
}
