use super::{Node, NodeValue};
use logger::terr;
use p2p_identity::addr::{Addr, KnownAddr};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub struct AddrsIterator {
    known_addrs_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
    known_addrs_rx: Arc<Mutex<UnboundedReceiver<Arc<Mutex<Node>>>>>,
}

pub struct AddrGuard {
    known_addr: KnownAddr,
    _node: Arc<Mutex<Node>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
        known_addrs_rx: Arc<Mutex<UnboundedReceiver<Arc<Mutex<Node>>>>>,
    ) -> AddrsIterator {
        AddrsIterator {
            known_addrs_tx,
            known_addrs_rx,
        }
    }

    pub async fn next(&self) -> Option<AddrGuard> {
        let mut rx = self.known_addrs_rx.lock().await;

        match rx.recv().await {
            Some(n) => {
                let node_lock = n.lock().await;
                match &node_lock.value {
                    NodeValue::Valued(v) => {
                        if let Addr::Known(addr) = &v.addr {
                            let addr_guard = AddrGuard {
                                known_addrs_tx: self.known_addrs_tx.clone(),
                                known_addr: addr.clone(),
                                _node: n.clone(),
                            };

                            return Some(addr_guard);
                        } else {
                            terr!(
                                "p2p_discovery",
                                "table",
                                "Invalid address is popped out of known \
                                address queue"
                            );

                            return None;
                        }
                    }
                    _ => {
                        terr!(
                            "p2p_discovery",
                            "table",
                            "Invalid address is popped out of known address \
                            queue"
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

impl AddrGuard {
    pub fn get_known_addr(&self) -> &KnownAddr {
        &self.known_addr
    }
}

impl Drop for AddrGuard {
    fn drop(&mut self) {
        println!(
            "Node (known addr) has been used. We push it back to the queue"
        );

        match self.known_addrs_tx.send(self._node.clone()) {
            Ok(_) => (),
            Err(err) => {
                terr!(
                    "p2p_discovery",
                    "table",
                    "Known address cannot be queued again. There is \
                        something wrong in the unbounded mpsc channel, \
                        err: {}",
                    err,
                );
            }
        }
    }
}
