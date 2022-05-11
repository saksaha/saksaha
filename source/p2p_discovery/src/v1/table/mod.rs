mod iter;
mod node;

pub(crate) use self::node::*;
pub use iter::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    OwnedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

// const DISC_TABLE_CAPACITY: usize = 100;
const DISC_TABLE_CAPACITY: usize = 5;

/// TODO Table shall have Kademlia flavored buckets
pub(crate) struct Table {
    addr_map: RwLock<HashMap<String, Arc<RwLock<Node>>>>,
    addrs: RwLock<Vec<Arc<RwLock<Node>>>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<Node>>>>,
    known_addrs_rx: Arc<RwLock<UnboundedReceiver<Arc<RwLock<Node>>>>>,
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

        let addrs = {
            let mut v = Vec::with_capacity(disc_table_capacity);

            for _ in 0..disc_table_capacity {
                let n = Node::Empty;
                let n = Arc::new(RwLock::new(n));

                v.push(n);
            }

            RwLock::new(v)
        };

        let (known_addrs_tx, known_addrs_rx) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), Arc::new(RwLock::new(rx)))
        };

        let table = Table {
            addr_map,
            addrs,
            known_addrs_tx,
            known_addrs_rx,
        };

        Ok(table)
    }

    pub(crate) async fn get_mapped_node(
        &self,
        disc_endpoint: &String,
    ) -> Option<Arc<RwLock<Node>>> {
        let addr_map = self.addr_map.read().await;
        addr_map.get(disc_endpoint).map(|n| n.clone())
    }

    pub(crate) async fn get_mapped_node_lock(
        &self,
        disc_endpoint: &String,
    ) -> Option<(OwnedRwLockWriteGuard<Node>, Arc<RwLock<Node>>)> {
        let addr_map = self.addr_map.read().await;
        match addr_map.get(disc_endpoint) {
            Some(n) => {
                let node = n.clone();
                return Some((node.write_owned().await, n.clone()));
            }
            None => {
                return None;
            }
        };
    }

    pub(crate) async fn get_empty_node_lock(
        &self,
    ) -> Option<(OwnedRwLockWriteGuard<Node>, Arc<RwLock<Node>>)> {
        let addrs_lock = self.addrs.read().await;

        for node in addrs_lock.iter() {
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

    pub(crate) async fn add_known_node(
        &self,
        node: Arc<RwLock<Node>>,
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
        let addrs = self.addrs.read().await;
        for (idx, node) in addrs.iter().enumerate() {
            match node.try_read() {
                Ok(n) => {
                    println!("addr table elements [{}] - {:?}", idx, n);
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
        node: Arc<RwLock<Node>>,
    ) -> Option<Arc<RwLock<Node>>> {
        let mut addr_map = self.addr_map.write().await;
        addr_map.insert(disc_endpoint.clone(), node)
    }
}
