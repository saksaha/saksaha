use super::node::Node;
use super::peer::Peer;
use logger::tinfo;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

const PEER_TABLE_CAPACITY: usize = 50;

pub struct PeerTable {
    peers: Mutex<Vec<Arc<Mutex<Node>>>>,
    peers_map: Mutex<HashMap<String, Arc<Mutex<Peer>>>>,
}

impl PeerTable {
    pub async fn init(
        peer_table_capacity: Option<u16>,
    ) -> Result<PeerTable, String> {
        let capacity = match peer_table_capacity {
            Some(c) => c.into(),
            None => PEER_TABLE_CAPACITY,
        };

        let peers = {
            let mut v = Vec::with_capacity(capacity);

            for _ in 0..capacity {
                let n = Node::Empty;
                v.push(Arc::new(Mutex::new(n)));
            }

            Mutex::new(v)
        };

        let peers_map = {
            let m = HashMap::new();
            Mutex::new(m)
        };

        tinfo!(
            "peer",
            "",
            "Initializing peer table, capacity: {}",
            capacity
        );

        let ps = PeerTable { peers_map, peers };

        Ok(ps)
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
