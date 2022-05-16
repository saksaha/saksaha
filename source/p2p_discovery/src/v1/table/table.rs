use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    OwnedRwLockWriteGuard, RwLock,
};

use crate::AddrsIterator;

use super::{
    addr::Addr,
    slot::{Slot, SlotGuard},
    KnownAddrNode,
};

// const DISC_TABLE_CAPACITY: usize = 100;
const DISC_TABLE_CAPACITY: usize = 5;

/// TODO Table shall have Kademlia flavored buckets
pub(crate) struct Table {
    addr_map: RwLock<HashMap<String, Arc<RwLock<Addr>>>>,
    // addrs: RwLock<Vec<Arc<RwLock<AddrNode>>>>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
    slots_rx: RwLock<UnboundedReceiver<Arc<Slot>>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<KnownAddrNode>>>>,
    known_addrs_rx: Arc<RwLock<UnboundedReceiver<Arc<RwLock<KnownAddrNode>>>>>,
}

pub(crate) enum AddrSlot {
    Slot(SlotGuard),
    Addr(OwnedRwLockWriteGuard<Addr>, Arc<RwLock<Addr>>),
}

impl Table {
    pub(crate) async fn init(
        disc_table_capacity: Option<u16>,
    ) -> Result<Table, String> {
        let addr_map = {
            let m = HashMap::new();
            RwLock::new(m)
        };

        let disc_table_capacity = match disc_table_capacity {
            Some(c) => c.into(),
            None => DISC_TABLE_CAPACITY,
        };

        // let addrs = {
        //     let mut v = Vec::with_capacity(disc_table_capacity);

        //     for _ in 0..disc_table_capacity {
        //         let n = AddrNode::Empty;
        //         let n = Arc::new(RwLock::new(n));

        //         v.push(n);
        //     }

        //     RwLock::new(v)
        // };

        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let slots_tx = Arc::new(tx);
            let slots_rx = RwLock::new(rx);

            for idx in 0..disc_table_capacity {
                let slot = {
                    let s = Slot { idx };
                    Arc::new(s)
                };

                slots_tx.send(slot);
            }

            (slots_tx, slots_rx)
        };

        let (known_addrs_tx, known_addrs_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), Arc::new(RwLock::new(rx)))
        };

        let table = Table {
            addr_map,
            // addrs,
            slots_tx,
            slots_rx,
            known_addrs_tx,
            known_addrs_rx,
        };

        Ok(table)
    }

    pub(crate) async fn get_mapped_node(
        &self,
        disc_endpoint: &String,
    ) -> Option<Arc<RwLock<Addr>>> {
        let addr_map = self.addr_map.read().await;
        addr_map.get(disc_endpoint).map(|n| n.clone())
    }

    pub(crate) async fn get_mapped_addr_lock(
        &self,
        disc_endpoint: &String,
    ) -> Option<(OwnedRwLockWriteGuard<Addr>, Arc<RwLock<Addr>>)> {
        let addr_map = self.addr_map.read().await;

        match addr_map.get(disc_endpoint) {
            Some(addr) => {
                let addr_lock = addr.clone().write_owned().await;

                return Some((addr_lock, addr.clone()));
            }
            None => {
                return None;
            }
        };
    }

    pub(crate) async fn get_empty_slot(&self) -> Result<SlotGuard, String> {
        let mut slots_rx = self.slots_rx.write().await;

        match slots_rx.recv().await {
            Some(slot) => {
                let slot_guard = SlotGuard {
                    slot,
                    slots_tx: self.slots_tx.clone(),
                };

                return Ok(slot_guard);
            }
            None => {
                return Err(format!(
                "All slots channels have been closed. Unexpected circumstance",
            ))
            }
        }

        // for node in addrs_lock.iter() {
        //     let node_lock = match node.clone().try_write_owned() {
        //         Ok(n) => n,
        //         Err(_) => {
        //             continue;
        //         }
        //     };

        //     if node_lock.is_empty() {
        //         return Some((node_lock, node.clone()));
        //     }
        // }

        // None
    }

    pub(crate) async fn add_known_node(
        &self,
        node: Arc<RwLock<KnownAddrNode>>,
    ) -> Result<(), String> {
        match self.known_addrs_tx.send(node) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "Couldn't push known node into queue, err: {}",
                err
            )),
        }
    }

    // For debugging purpose
    pub(crate) async fn print_all_nodes(&self) {
        // let addrs = self.addrs.read().await;
        let addr_map = self.addr_map.read().await;

        for (idx, addr) in addr_map.values().enumerate() {
            match addr.try_read() {
                Ok(addr) => {
                    println!("addr table elements [{}] - {}", idx, addr,);
                }
                Err(_err) => {
                    println!("addr table elements [{}] is locked", idx);
                }
            }
        }
    }

    pub(crate) fn iter(&self) -> AddrsIterator {
        AddrsIterator::init(
            self.known_addrs_tx.clone(),
            self.known_addrs_rx.clone(),
        )
    }

    pub async fn insert_mapping(
        &self,
        disc_endpoint: &String,
        addr: Arc<RwLock<Addr>>,
    ) -> Option<Arc<RwLock<Addr>>> {
        let mut addr_map = self.addr_map.write().await;

        addr_map.insert(disc_endpoint.clone(), addr)
    }
}
