use super::{Node, NodeValue};
use logger::terr;
use p2p_identity::addr::Addr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AddrsIterator {
    curr_idx: Mutex<usize>,
    known_addrs: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>,
    disc_table_capacity: usize,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs: Arc<Mutex<Vec<Arc<Mutex<Node>>>>>,
        disc_table_capacity: usize,
    ) -> AddrsIterator {
        AddrsIterator {
            curr_idx: Mutex::new(0),
            known_addrs,
            disc_table_capacity,
        }
    }

    pub async fn next(&self) -> Option<Addr> {
        let known_addrs = self.known_addrs.lock().await;
        let mut curr_idx = self.curr_idx.lock().await;

        println!(
            "next(): curr_idx: {}, known_addrs.len: {}",
            *curr_idx,
            known_addrs.len(),
        );

        if let Some(n) = known_addrs.get(*curr_idx) {
            let node_lock = n.lock().await;

            println!(
                "next(): found node, curr_idx: {}, known_addrs.len: {}",
                *curr_idx,
                known_addrs.len()
            );

            if let NodeValue::Valued(v) = &node_lock.value {
                *curr_idx = (*curr_idx + 1) % self.disc_table_capacity;
                return Some(v.addr.clone());
            }
        }

        *curr_idx = (*curr_idx + 1) % self.disc_table_capacity;
        None
    }
}
