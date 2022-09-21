use super::{
    addr::DiscAddr,
    slot::{Slot, SlotGuard},
};
use crate::AddrsIterator;
use colored::Colorize;
use sak_logger::debug;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    Mutex, OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock,
};

pub(crate) type PublicKey = String;
pub(crate) type AddrMap = HashMap<PublicKey, Arc<DiscAddr>>;

// const DISC_TABLE_CAPACITY: usize = 100;
const DISC_TABLE_CAPACITY: usize = 5;

/// TODO Table shall have Kademlia flavored buckets later on
pub struct AddrTable {
    addr_map: Arc<RwLock<AddrMap>>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
    slots_rx: RwLock<UnboundedReceiver<Arc<Slot>>>,
    known_addrs_tx: Arc<Sender<Arc<DiscAddr>>>,
    known_addrs_rx: Arc<RwLock<Receiver<Arc<DiscAddr>>>>,
    addr_recycle_tx: Arc<UnboundedSender<Arc<DiscAddr>>>,
    addrs_it_mutex: Arc<Mutex<usize>>,
}

impl AddrTable {
    pub(crate) async fn init(
        disc_table_capacity: Option<u16>,
    ) -> Result<AddrTable, String> {
        let addr_map = {
            let m = HashMap::new();

            Arc::new(RwLock::new(m))
        };

        let disc_table_capacity = match disc_table_capacity {
            Some(c) => c.into(),
            None => DISC_TABLE_CAPACITY,
        };

        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            let slots_tx = Arc::new(tx);
            let slots_rx = RwLock::new(rx);

            for _idx in 0..disc_table_capacity {
                let slot = {
                    let s = Slot { _idx };
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

        let addrs_it_mutex = Arc::new(Mutex::new(0));

        let table = AddrTable {
            addr_map,
            slots_tx,
            slots_rx,
            known_addrs_tx,
            known_addrs_rx,
            addr_recycle_tx,
            addrs_it_mutex,
        };

        Ok(table)
    }

    pub async fn get_mapped_addr(
        &self,
        public_key_str: &String,
    ) -> Option<Arc<DiscAddr>> {
        let addr_map = self.addr_map.read().await;

        addr_map.get(public_key_str).map(|n| n.clone())
    }

    pub(crate) async fn get_empty_slot(&self) -> Result<SlotGuard, String> {
        let mut slots_rx = self.slots_rx.write().await;

        match slots_rx.recv().await {
            Some(_slot) => {
                let slot_guard = SlotGuard {
                    _slot,
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
        node: Arc<DiscAddr>,
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
        let addr_map = self.addr_map.read().await;

        for (idx, addr) in addr_map.values().enumerate() {
            println!("addr table elements [{}] - {}", idx, addr,);
        }
    }

    pub fn new_iter(&self) -> Result<AddrsIterator, String> {
        let addrs_it_lock = match self.addrs_it_mutex.clone().try_lock_owned() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "Addr iter is already being used by some entity, err: {}",
                    err,
                ));
            }
        };

        let it =
            AddrsIterator::init(self.known_addrs_rx.clone(), addrs_it_lock);

        Ok(it)
    }

    pub(crate) async fn insert_mapping(
        &self,
        addr: Arc<DiscAddr>,
    ) -> Result<Option<Arc<DiscAddr>>, String> {
        // let mut addr_map = self.addr_map.write().await;
        let mut addr_map = self.get_addr_map_write().await;

        let key = &addr.known_addr.public_key_str;

        debug!(
            "Disc table insert mapping! key: {}, value: (p2p_ep: {})",
            addr.known_addr.get_public_ket_short().green(),
            addr.known_addr.get_p2p_endpoint(),
        );

        match self.enqueue_known_addr(addr.clone()).await {
            Ok(_) => {}
            Err(err) => {
                return Err(format!(
                    "Fail to insert mapping. Queue might have been closed, \
                        err: {}",
                    err,
                ));
            }
        };

        return Ok(addr_map.insert(key.to_string(), addr.clone()));
    }

    pub(crate) async fn get_addr_map_write(
        &self,
    ) -> OwnedRwLockWriteGuard<AddrMap> {
        let addr_map = self.addr_map.clone().write_owned().await;

        addr_map
    }

    pub(crate) async fn get_addr_map_read(
        &self,
    ) -> OwnedRwLockReadGuard<AddrMap> {
        let addr_map = self.addr_map.clone().read_owned().await;

        addr_map
    }

    pub(crate) async fn remove_mapping(
        &self,
        public_key_str: &String,
    ) -> Option<Arc<DiscAddr>> {
        let mut addr_map = self.addr_map.write().await;

        addr_map.remove(public_key_str)
    }

    pub async fn get_status(&self) -> Vec<String> {
        let addr_map = self.addr_map.read().await;
        let mut addr_vec = Vec::new();

        for (idx, addr) in addr_map.values().enumerate() {
            println!("addr table elements [{}] - {}", idx, addr,);

            addr_vec.push(addr.known_addr.get_disc_endpoint())
        }

        addr_vec
    }
}
