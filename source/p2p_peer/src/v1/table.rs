use super::node::Node;
use crate::{NodeGuard, NodeStatus, NodeValue};
use logger::{terr, tinfo};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

const PEER_TABLE_CAPACITY: usize = 50;

pub struct PeerTable {
    peers: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>,
    peers_map: Arc<Mutex<HashMap<String, Arc<Mutex<Node>>>>>,
    node_retreival_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
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

            let retrival_routine = RetrievalRoutine {};
            tokio::spawn(async move {
                retrival_routine.run(rx).await;
            });

            Arc::new(tx)
        };

        let peers = {
            let mut v = Vec::with_capacity(capacity);

            for _ in 0..capacity {
                let n = Node {
                    value: NodeValue::Empty,
                    status: NodeStatus::Available,
                    node_retrieval_tx: node_retreival_tx.clone(),
                };

                v.push(Arc::new(Mutex::new(n)));
            }

            Arc::new(Mutex::new(v))
        };

        let peers_map = {
            let m = HashMap::new();

            Arc::new(Mutex::new(m))
        };

        tinfo!(
            "peer",
            "",
            "Initializing peer table, capacity: {}",
            capacity
        );

        let ps = PeerTable {
            peers_map,
            peers,
            node_retreival_tx,
        };

        Ok(ps)
    }

    pub async fn get(
        &self,
        public_key: &String,
    ) -> Option<Result<NodeGuard, String>> {
        let peers_map = self.peers_map.clone();
        let peers_map_lock = peers_map.lock().await;

        match peers_map_lock.get(public_key) {
            Some(n) => {
                let node_lock = n.lock().await;
                if !node_lock.is_used() {
                    let g = NodeGuard { node: n.clone() };
                    return Some(Ok(g));
                } else {
                    return Some(Err(format!(
                        "Peer node is already being used"
                    )));
                }
            }
            None => {
                return None;
            }
        };
    }

    pub async fn reserve(
        &self,
        public_key: &String,
    ) -> Result<NodeGuard, String> {
        let peers = self.peers.lock().await;
        for node in peers.iter() {
            let mut node_lock = match node.try_lock() {
                Ok(n) => n,
                Err(_) => {
                    continue;
                }
            };

            node_lock.status = NodeStatus::Used;

            if node_lock.is_empty() && !node_lock.is_used() {
                let g = NodeGuard { node: node.clone() };

                let peers_map = self.peers_map.clone();
                let mut peers_map_lock = peers_map.lock().await;
                peers_map_lock.insert(public_key.clone(), node.clone());

                return Ok(g);
            }
        }

        Err(format!("Could not reserve a peer node"))

        // match peers_map_lock.(public_key) {
        //     Some(n) => Some(n.clone()),
        //     None => None,
        // }
    }

    // pub async fn reserve(&self) -> Result<Arc<Peer>, String> {
    //     let mut slots_rx_guard = self.slots_rx.lock().await;

    //     match slots_rx_guard.recv().await {
    //         Some(p) => Ok(p),
    //         None => Err(format!("Slots channel might be closed")),
    //     }
    // }

    // pub async fn register(&self, peer: Arc<Peer>) {
    //     let mut map = self.map.lock().await;

    //     let peer_val = peer.value.lock().await;
    //     if let PeerValue::Registered(p) = &*peer_val {
    //         let peer_id = p.transport.peer_id;
    //         map.insert(peer_id, peer.clone());

    //         debug!("Peer store added peer_id: {:?}", peer_id);
    //     } else {
    //     }
    // }

    // pub async fn DEMO__add(&self, peer: Arc<Peer>) {
    //     let list = self._list.lock().await;
    //     let found = false;
    //     for peer in list.iter() {
    //         let p_value = peer.value.lock().await;
    //         // p_value.
    //     }
    // }
}

pub struct RetrievalRoutine;

impl RetrievalRoutine {
    pub async fn run(&self, mut node_rx: UnboundedReceiver<Arc<Mutex<Node>>>) {
        loop {
            let node = match node_rx.recv().await {
                Some(n) => n,
                None => {
                    terr!(
                        "p2p_peer",
                        "table",
                        "All node guard senders have been closed. \
                        Something is critically wrong",
                    );

                    return;
                }
            };

            let mut n = node.lock().await;
            n.status = NodeStatus::Available;
        }
    }
}
