use crate::{Peer, PeerIterator, Slot, SlotGuard};
use colored::Colorize;
use sak_logger::{tdebug, terr, tinfo};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLock,
};

// const PEER_TABLE_CAPACITY: usize = 50;
const PEER_TABLE_CAPACITY: usize = 5;

pub type PublicKey = String;

pub struct PeerTable {
    peer_map: RwLock<HashMap<PublicKey, Arc<Peer>>>,
    slots_rx: RwLock<UnboundedReceiver<Arc<Slot>>>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
    peers_tx: Arc<UnboundedSender<Arc<Peer>>>,
    peer_it: Arc<RwLock<PeerIterator>>,
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
                            "p2p_peer_table",
                            "table",
                            "slots channel has been closed, err: {}",
                            err,
                        );
                    }
                };
            }

            (slots_tx, slots_rx)
        };

        let (peers_tx, peer_it) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let peers_tx = Arc::new(tx);

            let it = PeerIterator { peers_rx: rx };

            (peers_tx, Arc::new(RwLock::new(it)))
        };

        let peer_map = {
            let m = HashMap::new();

            RwLock::new(m)
        };

        let ps = PeerTable {
            peer_map,
            slots_rx,
            slots_tx,
            // peers_rx,
            peers_tx,
            peer_it,
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
    ) -> Option<Arc<Peer>> {
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

    // pub async fn get_mapped_peer_lock(
    //     &self,
    //     public_key: &PublicKey,
    // ) -> Option<OwnedRwLockWriteGuard<Peer>> {
    //     let peers_map_lock = self.peer_map.write().await;

    //     match peers_map_lock.get(public_key) {
    //         Some(n) => {
    //             let node = n.clone().write_owned().await;

    //             return Some(node);
    //         }
    //         None => {
    //             return None;
    //         }
    //     };
    // }

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
        peer: Arc<Peer>,
    ) -> Result<Option<Arc<Peer>>, String> {
        let public_key_str = peer.public_key_str.clone();

        tdebug!(
            "p2p_peer_table",
            "table",
            "Peer table insert mapping, her_public_key: {},",
            public_key_str.green(),
        );

        if let Err(err) = self.peers_tx.send(peer.clone()) {
            return Err(format!(
                "Cannot send to peer queue, rx might have been closed, err: {}",
                err,
            ));
        }

        let mut peer_map = self.peer_map.write().await;
        Ok(peer_map.insert(public_key_str, peer))
    }

    pub async fn get_status(&self) -> Vec<String> {
        let mut peer_vec = Vec::new();
        let peer_map = self.peer_map.read().await;

        for (_, peer) in peer_map.values().enumerate() {
            peer_vec.push(peer.addr.known_addr.p2p_endpoint().clone());
        }

        peer_vec
    }

    pub fn new_iter(&self) -> Arc<RwLock<PeerIterator>> {
        self.peer_it.clone()
    }
}
