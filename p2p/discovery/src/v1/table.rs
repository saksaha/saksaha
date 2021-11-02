use super::address::Address;
use log::{debug, error, info, warn};
use rand::prelude::*;
use saksaha_crypto::Signature;
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard, OwnedMutexGuard,
};

const CAPACITY: usize = 32;

type PeerId = [u8; PUBLIC_KEY_LEN];
type Nodes = HashMap<PeerId, Arc<TableNode>>;

pub struct Table {
    map: Mutex<Nodes>,
    keys: Mutex<HashSet<PeerId>>,
    rng: Mutex<StdRng>,
    slots_tx: Sender<Arc<TableNode>>,
    slots_rx: Mutex<Receiver<Arc<TableNode>>>,
}

impl Table {
    pub async fn init() -> Result<Table, String> {
        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::channel::<Arc<TableNode>>(CAPACITY);

            for _ in 0..CAPACITY {
                let empty_node = Arc::new(TableNode {
                    inner: Mutex::new(TableNodeInner::Empty),
                });

                match tx.send(empty_node).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Can't send empty TableNode to the pool, err: {}",
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

        let table = Table {
            map: Mutex::new(map),
            keys: Mutex::new(keys),
            rng: Mutex::new(rng),
            slots_tx,
            slots_rx: Mutex::new(slots_rx),
        };

        Ok(table)
    }

    pub async fn find(&self, peer_id: &PeerId) -> Option<Arc<TableNode>> {
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
    ) -> Result<Arc<TableNode>, String> {
        match self.find(peer_id).await {
            Some(n) => return Ok(n),
            None => return self.reserve().await,
        };
    }

    pub async fn find_or_try_reserve(
        &self,
        peer_id: &PeerId,
    ) -> Result<Arc<TableNode>, String> {
        match self.find(peer_id).await {
            Some(n) => return Ok(n),
            None => return self.try_reserve().await,
        };
    }

    pub async fn add<F>(
        &self,
        table_node: Arc<TableNode>,
        updater: F,
    ) -> Result<([u8; PUBLIC_KEY_LEN], String), String>
    where
        F: Fn(MutexGuard<TableNodeInner>) -> MutexGuard<TableNodeInner>,
    {
        let mut map = self.map.lock().await;
        let mut keys = self.keys.lock().await;

        let inner = table_node.inner.lock().await;
        let inner = updater(inner);

        let (public_key_bytes, endpoint) = if let TableNodeInner::Identified {
            public_key_bytes,
            ref addr,
            ..
        } = *inner
        {
            (public_key_bytes, addr.endpoint())
        } else {
            return Err(format!("Empty node can't be updated"));
        };

        std::mem::drop(inner);

        map.insert(public_key_bytes, table_node);
        keys.insert(public_key_bytes);


        Ok((public_key_bytes, endpoint))
    }

    // pub async fn next(&self) -> Option<TableNode> {
    //     let map = self.map.lock().await;
    //     let keys = self.keys.lock().await;
    //     let mut rng = self.rng.lock().await;
    //     let seed: usize = rng.gen();

    //     for i in 0..3 {
    //         let idx = (seed + i) % keys.len();
    //         let key = match keys.get(idx) {
    //             Some(k) => k,
    //             None => {
    //                 error!("Table key of idx: {}, not found", idx);
    //                 continue;
    //             }
    //         };

    //         let node = match map.get(key) {
    //             Some(n) => n.clone(),
    //             None => {
    //                 error!(
    //                     "None TableNode, something might be wrong, idx: {}",
    //                     idx,
    //                 );
    //                 return None;
    //             }
    //         };

    //         return Some(node);
    //     }

    //     None
    // }

    pub async fn reserve(&self) -> Result<Arc<TableNode>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.recv().await {
            Some(n) => return Ok(n),
            None => return Err(format!("Can't retrieve tableNode from pool")),
        };
    }

    pub async fn try_reserve(&self) -> Result<Arc<TableNode>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.try_recv() {
            Ok(n) => Ok(n),
            Err(err) => Err(format!(
                "Can't reserve a tableNode. Table might be busy, err: {}",
                err
            )),
        }
    }
}

pub struct TableNode {
    inner: Mutex<TableNodeInner>,
}

pub enum TableNodeInner {
    Empty,

    Identified {
        addr: Address,
        sig: Signature,
        p2p_port: u16,
        public_key_bytes: [u8; PUBLIC_KEY_LEN],
    },
}
