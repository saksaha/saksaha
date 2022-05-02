mod node;

pub(crate) use self::node::*;
use crate::{iterator::Iterator, CAPACITY};
use crypto::Signature;
use logger::tdebug;
use logger::twarn;
use p2p_identity::{
    addr::Addr,
    peer::{KnownPeer, UnknownPeer},
};
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, error::TrySendError, Receiver, Sender},
    Mutex, MutexGuard,
};

const ADDRS_MAX_COUNT: usize = 100;

// type Nodes = HashMap<PeerId, Arc<Node>>;

/// TODO Table shall have Kademlia flavored buckets
pub(crate) struct Table {
    addr_map: Arc<Mutex<HashMap<String, Node>>>,
    addrs: Arc<Mutex<Vec<Node>>>,
}

impl Table {
    pub async fn init(
        disc_table_capacity: Option<u16>,
    ) -> Result<Table, String> {
        let addr_map = {
            let m = HashMap::new();
            Arc::new(Mutex::new(m))
        };

        let addrs = {
            let mut v = Vec::with_capacity(ADDRS_MAX_COUNT);

            for _ in 0..ADDRS_MAX_COUNT {
                let n = Node::Empty;
                v.push(n);
            }

            Arc::new(Mutex::new(v))
        };

        let table = Table { addr_map, addrs };

        Ok(table)
    }

    pub async fn upsert(
        &self,
        addr: Addr,
    ) -> Result<Arc<Mutex<NodeValue>>, String> {
        let endpoint = addr.disc_endpoint();

        let addr_map = self.addr_map.clone();
        let mut addr_map_guard = addr_map.lock().await;

        // if map already had the address node
        match addr_map_guard.get(&endpoint) {
            Some(n) => {
                match &*n {
                    Node::Empty => {}
                    Node::Valued(a) => {
                        let addr = a.clone();
                        return Ok(addr);
                    }
                };
            }
            None => {}
        }

        let node_value = Arc::new(Mutex::new(NodeValue::new(addr)));
        let node = { Node::Valued(node_value.clone()) };

        addr_map_guard.insert(endpoint, node);

        return Ok(node_value);
    }

    // pub async fn add<F>(
    //     &self,
    //     table_node: Arc<Node>,
    //     updater: F,
    // ) -> Result<([u8; PUBLIC_KEY_LEN], String), String>
    // where
    //     F: Fn(MutexGuard<NodeValue>) -> MutexGuard<NodeValue>,
    // {
    //     let mut map = self.map.lock().await;
    //     let mut keys = self.keys.lock().await;

    //     let value_guard = table_node.value.lock().await;
    //     let mut value_guard = updater(value_guard);

    //     let identified_val = match value_guard.identified_mut() {
    //         Some(v) => v,
    //         None => {
    //             return Err(format!("Empty node can't be updated"));
    //         }
    //     };

    //     let public_key = identified_val.public_key;
    //     let endpoint = identified_val.addr.disc_endpoint();

    //     std::mem::drop(value_guard);

    //     map.insert(public_key, table_node.clone());
    //     keys.insert(public_key);
    //     match self.updates_tx.send(table_node).await {
    //         Ok(_) => (),
    //         Err(err) => {
    //             return Err(format!(
    //                 "Can't add Node to 'update' pool, endpoint: {}, err: {}",
    //                 endpoint, err,
    //             ))
    //         }
    //     };

    //     Ok((public_key, endpoint))
    // }

    // pub fn put_back(
    //     &self,
    //     node: Arc<Node>,
    // ) -> Result<(), TrySendError<Arc<Node>>> {
    //     match self.slots_tx.try_send(node) {
    //         Ok(_) => Ok(()),
    //         Err(err) => return Err(err),
    //     }
    // }

    // pub async fn try_reserve(&self) -> Result<Arc<Node>, String> {
    //     let mut slots_rx = self.slots_rx.lock().await;

    //     match slots_rx.try_recv() {
    //         Ok(n) => Ok(n),
    //         Err(err) => Err(format!(
    //             "Can't reserve a Node. Table might be busy, err: {}",
    //             err
    //         )),
    //     }
    // }

    // pub fn iter(&self) -> Arc<Iterator> {
    //     self.iter.clone()
    // }
}

// #[derive(Debug)]
// pub struct Node {
//     value: Arc<Mutex<NodeValue>>,
// }

// impl Node {
//     pub fn new_empty() -> Node {
//         Node {
//             value: Arc::new(Mutex::new(NodeValue::Empty)),
//         }
//     }

//     pub async fn get_value(&self) -> Option<IdentifiedValue> {
//         let val = self.value.lock().await;

//         val.identified()
//     }
// }

// /// TODO Node, when dropped either due to success or failure, needs to be
// /// checked to see if it should go to slots.
// impl Drop for Node {
//     fn drop(&mut self) {
//         tdebug!("p2p_discovery", "", "Node dropped");
//     }
// }

// #[derive(Clone, Debug)]
// pub struct IdentifiedValue {
//     pub addr: Address,
//     pub sig: Signature,
//     pub p2p_port: u16,
//     pub public_key: PeerId,
// }

// #[derive(Debug)]
// pub enum NodeValue {
//     Empty,

//     Identified(IdentifiedValue),
// }

// impl NodeValue {
//     pub fn new_identified(
//         addr: Address,
//         sig: Signature,
//         p2p_port: u16,
//         public_key: PeerId,
//     ) -> NodeValue {
//         NodeValue::Identified(IdentifiedValue {
//             addr,
//             sig,
//             p2p_port,
//             public_key,
//         })
//     }

//     fn identified(&self) -> Option<IdentifiedValue> {
//         if let NodeValue::Identified(v) = self {
//             Some(v.clone())
//         } else {
//             None
//         }
//     }

//     pub fn identified_mut(&mut self) -> Option<&mut IdentifiedValue> {
//         if let NodeValue::Identified(v) = self {
//             Some(v)
//         } else {
//             None
//         }
//     }
// }
