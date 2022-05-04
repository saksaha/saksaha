use super::{Node, NodeValue};
use logger::terr;
use p2p_identity::addr::Addr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

pub struct AddrsIterator {
    curr_idx: Mutex<usize>,
    known_addrs_tx: Arc<Sender<Arc<Mutex<Node>>>>,
    known_addrs_rx: Arc<Mutex<Receiver<Arc<Mutex<Node>>>>>,
    disc_table_capacity: usize,
}

pub struct Item {
    val: Addr,
    node: Arc<Mutex<Node>>,
    known_addrs_tx: Arc<Sender<Arc<Mutex<Node>>>>,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs_tx: Arc<Sender<Arc<Mutex<Node>>>>,
        known_addrs_rx: Arc<Mutex<Receiver<Arc<Mutex<Node>>>>>,
        disc_table_capacity: usize,
    ) -> AddrsIterator {
        AddrsIterator {
            curr_idx: Mutex::new(0),
            known_addrs_tx,
            known_addrs_rx,
            disc_table_capacity,
        }
    }

    pub async fn next(&self) -> Option<Item> {
        let mut rx = self.known_addrs_rx.lock().await;

        match rx.recv().await {
            Some(n) => {
                let node_lock = n.lock().await;
                match &node_lock.value {
                    NodeValue::Valued(v) => {
                        let item = Item {
                            known_addrs_tx: self.known_addrs_tx.clone(),
                            val: v.addr.clone(),
                            node: n.clone(),
                        };

                        return Some(item);
                    }
                    _ => {
                        terr!(
                            "p2p_discovery",
                            "table",
                            "Known addr is empty. Something is wrong"
                        );

                        return None;
                    }
                };
            }
            None => {
                terr!(
                    "p2p_discovery",
                    "table",
                    "Known addrs queue has been closed. Coudn't retrieve \
                    known address.",
                );

                return None;
            }
        };
    }
}

impl Item {
    pub fn get_value(&self) -> &Addr {
        &self.val
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        // self.known_addrs_tx.send(self.node.clone()).await;
    }
}
