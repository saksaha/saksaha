use super::address::Address;
use crate::{CAPACITY, iterator::Iterator};
use futures::Future;
use log::{debug, error, info, warn};
use rand::prelude::*;
use saksaha_crypto::Signature;
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard, OwnedMutexGuard,
};

type PeerId = [u8; PUBLIC_KEY_LEN];
type Nodes = HashMap<PeerId, Arc<TableNode>>;

pub(crate) struct Table {
    map: Mutex<Nodes>,
    keys: Mutex<HashSet<PeerId>>,
    rng: Mutex<StdRng>,
    slots_tx: Sender<Arc<TableNode>>,
    slots_rx: Mutex<Receiver<Arc<TableNode>>>,
    updates_tx: Sender<Arc<TableNode>>,
    updates_rx: Mutex<Receiver<Arc<TableNode>>>,
    iter: Arc<Iterator>,
}

impl Table {
    pub async fn init() -> Result<Table, String> {
        let iter = {
            let it = Iterator::new();
            Arc::new(it)
        };

        let (updates_tx, updates_rx) = {
            let (tx, rx) = mpsc::channel(CAPACITY);
            (tx, rx)
        };

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
            updates_tx,
            updates_rx: Mutex::new(updates_rx),
            iter,
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

        map.insert(public_key_bytes, table_node.clone());
        keys.insert(public_key_bytes);
        match self.updates_tx.send(table_node).await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                "Can't add TableNode to 'update' pool, endpoint: {}, err: {}",
                endpoint, err,
            ))
            }
        };

        Ok((public_key_bytes, endpoint))
    }

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

    pub fn iter(&self) -> Arc<Iterator> {
        self.iter.clone()
    }

    // pub fn iter(&self) -> Iterator {
    //     Iterator {}
    // }

    // pub async fn next(&self) -> Result<Arc<TableNode>, String> {
    //     let mut updates_rx = self.updates_rx.lock().await;
    //     let tnode = match updates_rx.recv().await {
    //         Some(n) => n,
    //         None => {
    //             return Err(format!("Update channel is closed, fatal error"))
    //         }
    //     };

    //     Ok(tnode)
    // }
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
