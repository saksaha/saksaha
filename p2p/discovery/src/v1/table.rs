use super::address::Address;
use crate::{iterator::Iterator, CAPACITY};
use log::{debug, error, info, warn};
use rand::prelude::*;
use saksaha_crypto::Signature;
use saksaha_p2p_identity::{PUBLIC_KEY_LEN, PeerId};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard,
};

type Nodes = HashMap<PeerId, Arc<Node>>;

pub(crate) struct Table {
    map: Mutex<Nodes>,
    keys: Mutex<HashSet<PeerId>>,
    rng: Mutex<StdRng>,
    slots_tx: Sender<Arc<Node>>,
    slots_rx: Mutex<Receiver<Arc<Node>>>,
    updates_tx: Arc<Sender<Arc<Node>>>,
    updates_rx: Arc<Mutex<Receiver<Arc<Node>>>>,
    iter: Arc<Iterator>,
}

impl Table {
    pub async fn init() -> Result<Table, String> {
        let (updates_tx, updates_rx) = {
            let (tx, rx) = mpsc::channel(CAPACITY);
            (Arc::new(tx), Arc::new(Mutex::new(rx)))
        };

        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::channel::<Arc<Node>>(CAPACITY);

            for _ in 0..CAPACITY {
                let empty_node = Arc::new(Node::new_empty());

                match tx.send(empty_node).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Can't send empty Node to the pool, err: {}",
                            err
                        ));
                    }
                }
            }

            (tx, rx)
        };

        let map = HashMap::with_capacity(CAPACITY);
        let keys = HashSet::new();
        let rng = SeedableRng::from_entropy();
        let iter = {
            let it = Iterator::new(updates_tx.clone(), updates_rx.clone());
            Arc::new(it)
        };

        let table = Table {
            map: Mutex::new(map),
            keys: Mutex::new(keys),
            rng: Mutex::new(rng),
            slots_tx,
            slots_rx: Mutex::new(slots_rx),
            updates_tx,
            updates_rx,
            iter,
        };

        Ok(table)
    }

    pub async fn find(&self, peer_id: &PeerId) -> Option<Arc<Node>> {
        let map = self.map.lock().await;

        if let Some(n) = map.get(peer_id) {
            return Some(n.clone());
        } else {
            return None;
        }
    }

    pub async fn find_or_reserve(
        &self,
        peer_id: &PeerId,
    ) -> Result<Arc<Node>, String> {
        match self.find(peer_id).await {
            Some(n) => return Ok(n),
            None => return self.reserve().await,
        };
    }

    pub async fn find_or_try_reserve(
        &self,
        peer_id: &PeerId,
    ) -> Result<Arc<Node>, String> {
        match self.find(peer_id).await {
            Some(n) => return Ok(n),
            None => return self.try_reserve().await,
        };
    }

    pub async fn add<F>(
        &self,
        table_node: Arc<Node>,
        updater: F,
    ) -> Result<([u8; PUBLIC_KEY_LEN], String), String>
    where
        F: Fn(MutexGuard<NodeValue>) -> MutexGuard<NodeValue>,
    {
        let mut map = self.map.lock().await;
        let mut keys = self.keys.lock().await;

        let value_guard = table_node.value.lock().await;
        let mut value_guard = updater(value_guard);

        let identified_val = match value_guard.identified_mut() {
            Some(v) => v,
            None => {
                return Err(format!("Empty node can't be updated"));
            }
        };

        let public_key = identified_val.public_key;
        let endpoint = identified_val.addr.disc_endpoint();

        std::mem::drop(value_guard);

        map.insert(public_key, table_node.clone());
        keys.insert(public_key);
        match self.updates_tx.send(table_node).await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Can't add Node to 'update' pool, endpoint: {}, err: {}",
                    endpoint, err,
                ))
            }
        };

        Ok((public_key, endpoint))
    }

    pub async fn reserve(&self) -> Result<Arc<Node>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.recv().await {
            Some(n) => return Ok(n),
            None => return Err(format!("Can't retrieve Node from pool")),
        };
    }

    pub async fn try_reserve(&self) -> Result<Arc<Node>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.try_recv() {
            Ok(n) => Ok(n),
            Err(err) => Err(format!(
                "Can't reserve a Node. Table might be busy, err: {}",
                err
            )),
        }
    }

    pub fn iter(&self) -> Arc<Iterator> {
        self.iter.clone()
    }
}

pub struct Node {
    value: Arc<Mutex<NodeValue>>,
}

impl Node {
    pub fn new_empty() -> Node {
        Node {
            value: Arc::new(Mutex::new(NodeValue::Empty)),
        }
    }

    pub async fn get_value(&self) -> Option<IdentifiedValue> {
        let val = self.value.lock().await;

        val.identified()
    }
}

#[derive(Clone)]
pub struct IdentifiedValue {
    pub addr: Address,
    pub sig: Signature,
    pub p2p_port: u16,
    pub public_key: PeerId,
}

pub enum NodeValue {
    Empty,

    Identified(IdentifiedValue),
}

impl NodeValue {
    pub fn new_identified(
        addr: Address,
        sig: Signature,
        p2p_port: u16,
        public_key: PeerId,
    ) -> NodeValue {
        NodeValue::Identified(IdentifiedValue {
            addr,
            sig,
            p2p_port,
            public_key,
        })
    }

    fn identified(&self) -> Option<IdentifiedValue> {
        if let NodeValue::Identified(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn identified_mut(&mut self) -> Option<&mut IdentifiedValue> {
        if let NodeValue::Identified(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
