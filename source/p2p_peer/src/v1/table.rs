use crate::{Peer, Slot, SlotGuard};
use logger::{terr, tinfo};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    OwnedRwLockMappedWriteGuard, OwnedRwLockWriteGuard, RwLock,
};

// const PEER_TABLE_CAPACITY: usize = 50;
const PEER_TABLE_CAPACITY: usize = 5;

pub type PublicKey = String;

pub struct PeerTable {
    peer_map: RwLock<HashMap<PublicKey, Arc<RwLock<Peer>>>>,
    slots_rx: RwLock<UnboundedReceiver<Arc<Slot>>>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
}

pub enum PeerSlot {
    Slot(SlotGuard),
    Peer(OwnedRwLockWriteGuard<Peer>),
}

impl PeerTable {
    pub async fn init(
        peer_table_capacity: Option<u16>,
    ) -> Result<PeerTable, String> {
        let capacity = match peer_table_capacity {
            Some(c) => c.into(),
            None => PEER_TABLE_CAPACITY,
        };

        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let slots_tx = Arc::new(tx);
            let slots_rx = RwLock::new(rx);

            for idx in 0..capacity {
                let s = Slot { idx };

                match slots_tx.send(Arc::new(s)) {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "p2p_peer",
                            "table",
                            "slots channel has been closed, err: {}",
                            err,
                        );
                    }
                };
            }

            (slots_tx, slots_rx)
        };

        let peer_map = {
            let m = HashMap::new();

            RwLock::new(m)
        };

        let ps = PeerTable {
            peer_map,
            slots_rx,
            slots_tx,
        };

        tinfo!(
            "peer",
            "",
            "Initializing peer table, capacity: {}",
            capacity
        );

        Ok(ps)
    }

    pub async fn get_mapped_peer(
        &self,
        public_key: &PublicKey,
    ) -> Option<Arc<RwLock<Peer>>> {
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

    pub async fn get_mapped_peer_lock(
        &self,
        public_key: &PublicKey,
    ) -> Option<OwnedRwLockWriteGuard<Peer>> {
        let peers_map_lock = self.peer_map.write().await;

        match peers_map_lock.get(public_key) {
            Some(n) => {
                let node = n.clone().write_owned().await;

                return Some(node);
            }
            None => {
                return None;
            }
        };
    }

    pub async fn get_empty_slot(&self) -> Result<SlotGuard, String> {
        let mut slots_rx = self.slots_rx.write().await;

        match slots_rx.recv().await {
            Some(s) => {
                let slot_guard = SlotGuard {
                    slot: s,
                    slots_tx: self.slots_tx.clone(),
                };

                return Ok(slot_guard);
            }
            None => {
                return Err(format!(
                    "Unusual circumstance. Peer slots have been closed"
                ));
            }
        }
    }

    pub async fn insert_mapping(
        &self,
        public_key: &PublicKey,
        node: Arc<RwLock<Peer>>,
    ) -> Option<Arc<RwLock<Peer>>> {
        let mut peer_map = self.peer_map.write().await;
        peer_map.insert(public_key.clone(), node)
    }

    pub async fn get_status(&self) -> Vec<String> {
        let mut peer_vec = Vec::new();
        let peer_map = self.peer_map.read().await;

        for (idx, peer) in peer_map.values().enumerate() {
            match peer.try_read() {
                Ok(peer_lock) => match &peer_lock.addr_guard {
                    Some(addr_guard) => {
                        let addr_lock = addr_guard.addr.read().await;
                        peer_vec
                            .push(addr_lock.known_addr.p2p_endpoint().clone());
                        // let addr_val = &addr_lock.val;
                        // match addr_val {
                        //     AddrVal::Known(k) => {
                        //         peer_vec.push(k.p2p_endpoint().clone());
                        //     }
                        //     AddrVal::Unknown(u) => {
                        //         peer_vec.push(
                        //             u.p2p_endpoint().unwrap().clone(), //
                        //         );
                        //     }
                        // }
                    }

                    None => {
                        println!("error: cannot get addr_guard");
                    }
                },
                Err(_err) => {
                    println!("addr table elements [{}] is locked", idx);
                }
            }
        }

        peer_vec
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

// pub struct RecycleRoutine {}

// impl RecycleRoutine {
//     pub(super) async fn run(
//         &self,
//         mut slots_rx: UnboundedReceiver<Arc<RwLock<Peer>>>,
//         slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
//     ) {
//         loop {
//             let slot = match slots_rx.recv().await {
//                 Some(s) => s,
//                 None => {
//
//                 },
//             }
//         }
//     }
// }
