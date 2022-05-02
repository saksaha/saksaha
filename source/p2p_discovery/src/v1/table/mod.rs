mod node;

pub(crate) use self::node::*;
use p2p_identity::addr::Addr;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};

/// TODO Table shall have Kademlia flavored buckets
pub(crate) struct Table {
    addr_map: Arc<Mutex<HashMap<String, Arc<Mutex<Node>>>>>,
    addrs: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>,
}

impl Table {
    pub(crate) async fn init(
        disc_table_capacity: Option<u16>,
    ) -> Result<Table, String> {
        let addr_map = {
            let m = HashMap::new();
            Arc::new(Mutex::new(m))
        };

        let disc_table_capacity = match disc_table_capacity {
            Some(c) => c.into(),
            None => 100,
        };

        let addrs = {
            let mut v = Vec::with_capacity(disc_table_capacity);

            for _ in 0..disc_table_capacity {
                let n = Node {
                    value: NodeValue::Empty,
                };
                v.push(Arc::new(Mutex::new(n)));
            }

            Arc::new(Mutex::new(v))
        };

        let table = Table { addr_map, addrs };

        Ok(table)
    }

    pub(crate) async fn upsert(
        &self,
        addr: &Addr,
    ) -> Result<Arc<Mutex<Node>>, String> {
        let endpoint = addr.disc_endpoint();

        let addr_map = self.addr_map.clone();
        let mut addr_map_guard = addr_map.lock().await;

        // if map already had the address node
        if let Some(n) = addr_map_guard.get(&endpoint) {
            return Ok(n.clone());
        } else {
            // When the map doesn't have a node associated with the endpoint
            let addrs_lock = self.addrs.lock().await;

            match find_empty_node(&addrs_lock) {
                Some(n) => {
                    addr_map_guard.insert(endpoint, n.clone());

                    let mut node_lock = n.lock().await;
                    node_lock.value = NodeValue::Valued(NodeValueInner {
                        addr: addr.clone(),
                        status: NodeStatus::Initialized,
                    });

                    return Ok(n.clone());
                }
                None => {
                    return Err(format!("Every node is currently locked"));
                }
            };
        }
    }
}

fn find_empty_node(
    addrs_lock: &MutexGuard<Vec<Arc<Mutex<Node>>>>,
) -> Option<Arc<Mutex<Node>>> {
    for node in addrs_lock.iter() {
        match node.try_lock() {
            Ok(n) => match &n.value {
                NodeValue::Empty => {
                    return Some(node.clone());
                }
                _ => continue,
            },
            Err(_) => continue,
        };
    }

    return None;
}
