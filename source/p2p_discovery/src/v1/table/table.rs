use super::{
    addr::Addr,
    slot::{Slot, SlotGuard},
};
use crate::AddrsIterator;
use logger::{tdebug, terr};
use p2p_identity::addr::AddrStatus;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    OwnedRwLockWriteGuard, RwLock,
};

// const DISC_TABLE_CAPACITY: usize = 100;
const DISC_TABLE_CAPACITY: usize = 5;

/// TODO Table shall have Kademlia flavored buckets
pub(crate) struct Table {
    addr_map: RwLock<HashMap<String, Arc<RwLock<Addr>>>>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
    slots_rx: RwLock<UnboundedReceiver<Arc<Slot>>>,
    known_addrs_tx: Arc<Sender<Arc<RwLock<Addr>>>>,
    known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
    addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
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

        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let slots_tx = Arc::new(tx);
            let slots_rx = RwLock::new(rx);

            for idx in 0..disc_table_capacity {
                let slot = {
                    let s = Slot { idx };
                    Arc::new(s)
                };

                match slots_tx.send(slot) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Error initializing slots queue, err: {}",
                            err
                        ));
                    }
                };
            }

            (slots_tx, slots_rx)
        };

        let (known_addrs_tx, known_addrs_rx) = {
            let (tx, rx) = mpsc::channel(disc_table_capacity);
            (Arc::new(tx), Arc::new(RwLock::new(rx)))
        };

        let (addr_recycle_tx, addr_recycle_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), RwLock::new(rx))
        };

        let recycle_routine = RecycleRoutine {
            known_addrs_tx: known_addrs_tx.clone(),
            addr_recycle_rx,
        };
        tokio::spawn(async move {
            recycle_routine.run().await;
        });

        let table = Table {
            addr_map,
            slots_tx,
            slots_rx,
            known_addrs_tx,
            known_addrs_rx,
            addr_recycle_tx,
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
    }

    pub(crate) async fn enqueue_known_addr(
        &self,
        node: Arc<RwLock<Addr>>,
    ) -> Result<(), String> {
        match self.known_addrs_tx.send(node).await {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(format!("Could not enqueue known addr, err: {}", err))
            }
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

    pub(crate) fn new_iter(&self) -> AddrsIterator {
        AddrsIterator::init(
            self.addr_recycle_tx.clone(),
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

    pub async fn remove_mapping(
        &self,
        disc_endpoint: &String,
    ) -> Option<Arc<RwLock<Addr>>> {
        let mut addr_map = self.addr_map.write().await;

        addr_map.remove(disc_endpoint)
    }
}

struct RecycleRoutine {
    known_addrs_tx: Arc<Sender<Arc<RwLock<Addr>>>>,
    addr_recycle_rx: RwLock<UnboundedReceiver<Arc<RwLock<Addr>>>>,
}

impl RecycleRoutine {
    async fn run(&self) {
        let mut addr_recycle_rx = self.addr_recycle_rx.write().await;

        loop {
            match addr_recycle_rx.recv().await {
                Some(a) => {
                    let addr = a.read().await;

                    if let AddrStatus::Invalid { err } = addr.get_status() {
                        tdebug!(
                            "p2p_discovery",
                            "table",
                            "Addr is dropped and will not be recycled, \
                            addr: {}, err: {}",
                            addr,
                            err,
                        );
                    } else {
                        drop(addr);

                        match self.known_addrs_tx.send(a).await {
                            Ok(_) => (),
                            Err(err) => {
                                terr!(
                                    "p2p_discovery",
                                    "table",
                                    "Cannot push addr back to the queue. \
                                        Known addrs rx is closed, err: {}",
                                    err,
                                )
                            }
                        }
                    }
                }
                None => {
                    terr!(
                        "p2p_discovery",
                        "table",
                        "Addr recycle channel has been closed. \
                            All txs are gone."
                    );
                }
            }
        }
    }
}