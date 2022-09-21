use crate::{Peer, PeerIterator, PeerTableError, Runtime, Slot, SlotGuard};
use colored::Colorize;
use sak_logger::{debug, error, info};
use sak_p2p_addr::UnknownAddr;
use std::{
    collections::{hash_map::Values, HashMap},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLock,
};

const PEER_TABLE_CAPACITY: isize = 30;

pub type PublicKey = String;
pub type PeerMap = HashMap<PublicKey, Arc<Peer>>;

pub struct PeerTable {
    peer_map: Arc<RwLock<PeerMap>>,
    slots_rx: RwLock<UnboundedReceiver<Slot>>,
    slots_tx: Arc<UnboundedSender<Slot>>,
    peer_queue_tx: Arc<UnboundedSender<Arc<Peer>>>,
    peer_queue_iter: Arc<RwLock<PeerIterator>>,
}

impl PeerTable {
    pub async fn init(peer_table_capacity: Option<i16>) -> Result<PeerTable, PeerTableError> {
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

                match slots_tx.send(s) {
                    Ok(_) => (),
                    Err(err) => {
                        error!("slots channel has been closed, err: {}", err,);
                    }
                };
            }

            (slots_tx, slots_rx)
        };

        let (peer_queue_tx, peer_queue_iter) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let peers_tx = Arc::new(tx);

            let it = PeerIterator { peers_rx: rx };

            (peers_tx, Arc::new(RwLock::new(it)))
        };

        let peer_map = {
            let m = HashMap::new();

            Arc::new(RwLock::new(m))
        };

        let runtime = Runtime {
            peer_map: peer_map.clone(),
        };

        tokio::spawn(async move {
            runtime.run().await;
        });

        let ps = PeerTable {
            peer_map,
            slots_rx,
            slots_tx,
            peer_queue_tx,
            peer_queue_iter,
        };

        info!("Initializing peer table, capacity: {}", capacity);

        Ok(ps)
    }

    pub async fn get_mapped_peer(&self, public_key: &PublicKey) -> Option<Arc<Peer>> {
        let peers_map_lock = self.peer_map.read().await;

        match peers_map_lock.get(public_key) {
            Some(n) => {
                return Some(n.clone());
            }
            None => {
                return None;
            }
        };
    }

    pub async fn get_peer_addrs(&self) -> Vec<UnknownAddr> {
        let peers_map_lock = self.peer_map.read().await;

        let unknown_addrs = peers_map_lock
            .values()
            .map(|peer| peer.get_addr().downgrade())
            .collect();

        unknown_addrs
    }

    pub fn get_peer_map(&self) -> &Arc<RwLock<PeerMap>> {
        &self.peer_map
    }

    pub async fn get_empty_slot(&self) -> Result<SlotGuard, PeerTableError> {
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
                return Err(format!("Peer slots have beeen closed. Critical error").into());
            }
        }
    }

    pub async fn insert_mapping(
        &self,
        peer: Arc<Peer>,
    ) -> Result<Option<Arc<Peer>>, PeerTableError> {
        let public_key_str = peer.get_public_key().to_string();

        debug!(
            "Peer table insert mapping, her_public_key: {},",
            peer.get_public_key_short().green(),
        );

        let mut peer_map = self.peer_map.write().await;

        if let Err(err) = self.peer_queue_tx.send(peer.clone()) {
            return Err(format!(
                "Cannot send to peer queue, rx might have been closed, err: {}",
                err,
            )
            .into());
        }

        Ok(peer_map.insert(public_key_str, peer))
    }

    pub async fn get_status(&self) -> Vec<String> {
        let mut peer_vec = Vec::new();
        let peer_map = self.peer_map.read().await;

        for (_, peer) in peer_map.values().enumerate() {
            peer_vec.push(peer.get_addr().known_addr.get_p2p_endpoint().clone());
        }

        peer_vec
    }

    pub fn peer_queue_iter(&self) -> Arc<RwLock<PeerIterator>> {
        self.peer_queue_iter.clone()
    }
}
