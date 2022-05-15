use crate::Peer;
use logger::{terr, tinfo};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    OwnedRwLockMappedWriteGuard, OwnedRwLockWriteGuard, RwLock,
};

// const PEER_TABLE_CAPACITY: usize = 50;
const PEER_TABLE_CAPACITY: usize = 5;

pub type PublicKey = String;

pub struct PeerTable {
    peers: RwLock<Vec<Arc<RwLock<PeerNode>>>>,
    peer_map: RwLock<HashMap<PublicKey, Arc<RwLock<PeerNode>>>>,
    node_retreival_tx: Arc<UnboundedSender<Arc<RwLock<PeerNode>>>>,
}

pub enum PeerNode {
    Empty,
    Peer(Peer),
}

impl PeerNode {
    pub fn is_empty(&self) -> bool {
        if let PeerNode::Empty = &self {
            return true;
        } else {
            return false;
        }
    }
}

impl PeerTable {
    pub async fn init(
        peer_table_capacity: Option<u16>,
    ) -> Result<PeerTable, String> {
        let capacity = match peer_table_capacity {
            Some(c) => c.into(),
            None => PEER_TABLE_CAPACITY,
        };

        let node_retreival_tx = {
            let (tx, rx) = mpsc::unbounded_channel();

            Arc::new(tx)
        };

        let peers = {
            let mut v = Vec::with_capacity(capacity);

            for _ in 0..capacity {
                let n = PeerNode::Empty;

                v.push(Arc::new(RwLock::new(n)));
            }

            RwLock::new(v)
        };

        let peer_map = {
            let m = HashMap::new();

            RwLock::new(m)
        };

        tinfo!(
            "peer",
            "",
            "Initializing peer table, capacity: {}",
            capacity
        );

        let ps = PeerTable {
            peer_map,
            peers,
            node_retreival_tx,
        };

        Ok(ps)
    }

    pub async fn get_mapped_node(
        &self,
        public_key: &PublicKey,
    ) -> Option<Arc<RwLock<PeerNode>>> {
        let peers_map_lock = self.peer_map.write().await;

        match peers_map_lock.get(public_key) {
            Some(n) => {
                return Some(n.clone());
            }
            None => {
                return None;
            }
        };
    }

    pub async fn get_mapped_node_lock(
        &self,
        public_key: &PublicKey,
    ) -> Option<(OwnedRwLockWriteGuard<PeerNode>, Arc<RwLock<PeerNode>>)> {
        let peers_map_lock = self.peer_map.write().await;

        match peers_map_lock.get(public_key) {
            Some(n) => {
                let node = n.clone();
                return Some((node.write_owned().await, n.clone()));
            }
            None => {
                return None;
            }
        };
    }

    pub async fn get_empty_node_lock(
        &self,
    ) -> Option<(OwnedRwLockWriteGuard<PeerNode>, Arc<RwLock<PeerNode>>)> {
        let peers_lock = self.peers.write().await;

        for node in peers_lock.iter() {
            let node_lock = match node.clone().try_write_owned() {
                Ok(n) => n,
                Err(_) => {
                    continue;
                }
            };

            if node_lock.is_empty() {
                return Some((node_lock, node.clone()));
            }
        }

        None
    }

    pub async fn insert_mapping(
        &self,
        public_key: &PublicKey,
        node: Arc<RwLock<PeerNode>>,
    ) -> Option<Arc<RwLock<PeerNode>>> {
        let mut peer_map = self.peer_map.write().await;
        peer_map.insert(public_key.clone(), node)
    }

    // pub async fn print_all_nodes(&self) -> u16 {
    //     let peers = self.peers.lock().await;

    //     for (idx, node) in peers.iter().enumerate() {
    //         if let Ok(node_lock) = node.try_lock() {
    //             let a = &node_lock.value;
    //             match a {
    //                 NodeValue::Valued(p) => {
    //                     println!(
    //                         "peer table [{}] - p2p_port: {}",
    //                         idx, p.transport.p2p_port
    //                     );
    //                     return p.transport.p2p_port;
    //                 }
    //                 _ => {
    //                     println!("peer table [{}] - empty", idx);
    //                 }
    //             };
    //         } else {
    //             println!("peer table [{}] - locked", idx,);
    //         }
    //     }
    //     return 0;
    // }

    // pub async fn print_all_mapped_nodes(&self) {
    //     let peers_map = self.peers_map.lock().await;

    //     let len = peers_map.len();
    //     println!("Peer map length: {}", len);

    //     for (idx, node) in peers_map.values().into_iter().enumerate() {
    //         if let Ok(node_lock) = node.try_lock() {
    //             let a = &node_lock.value;
    //             match a {
    //                 NodeValue::Valued(p) => {
    //                     println!(
    //                         "peer table [{}] - p2p_port: {}",
    //                         idx, p.transport.p2p_port
    //                     );
    //                 }
    //                 _ => (),
    //             };
    //         }
    //     }
    // }
}
