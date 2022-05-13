use super::{Node, NodeValue, NodeValueInner};
use chrono::{DateTime, Utc};
use crypto::Signature;
use logger::{tdebug, terr};
use p2p_identity::addr::{Addr, KnownAddr, UnknownAddr};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub struct AddrsIterator {
    known_addrs_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
    known_addrs_rx: Arc<Mutex<UnboundedReceiver<Arc<Mutex<Node>>>>>,
}

pub struct AddrGuard {
    // known_addr: KnownAddr,
    _node: Arc<Mutex<Node>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
    pub x: DateTime<Utc>,
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

    pub async fn next(&self) -> Result<AddrGuard, String> {
        let mut rx = self.known_addrs_rx.lock().await;

        match rx.recv().await {
            Some(n) => {
                let node_lock = n.lock().await;

                match &node_lock.value {
                    NodeValue::Valued(v) => {
                        if let Addr::Known(addr) = &v.addr {
                            let x = Utc::now();
                            let addr_guard = AddrGuard {
                                known_addrs_tx: self.known_addrs_tx.clone(),
                                // known_addr: addr.clone(),
                                _node: n.clone(),
                                x,
                            };

                            println!(
                                "next(): known_at: {}, x: {}",
                                addr.known_at, x
                            );

                            return Ok(addr_guard);
                        } else {
                            return Err(format!(
                                "
                                Invalid address is popped out of known \
                                address queue"
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Invalid address is popped out of known address \
                            queue"
                        ));
                    }
                };
            }
            None => {
                return Err(format!(
                    "Known addrs queue has been closed. Coudn't retrieve \
                    known address.",
                ));
            }
        };
    }
}

impl AddrGuard {
    pub async fn get_known_addr(&self) -> KnownAddr {
        let node_lock = self._node.lock().await;
        match &node_lock.value {
            NodeValue::Valued(v) => match v.addr {
                Addr::Known(ref kn) => {
                    return kn.clone();
                }
                Addr::Unknown(_) => {
                    panic!("Unknown addr")
                }
            },
            _ => {
                panic!("empty addr");
            }
        }
    }

    // #[cfg(test)]
    pub fn new_dummy(
        public_key: k256::PublicKey,
        public_key_str: String,
        sig: Signature,
        disc_port: u16,
        p2p_port: u16,
    ) -> AddrGuard {
        let node = {
            Node {
                value: NodeValue::Valued(NodeValueInner {
                    addr: Addr::Known(KnownAddr {
                        ip: "0.0.0.0".to_string(),
                        disc_port: disc_port,
                        p2p_port: p2p_port,
                        sig,
                        public_key_str,
                        known_at: Utc::now(),
                        public_key,
                    }),
                    status: super::NodeStatus::Initialized,
                }),
            }
        };
        let (addrs_tx, _) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), Arc::new(Mutex::new(rx)))
        };

        AddrGuard {
            _node: Arc::new(Mutex::new(node)),
            known_addrs_tx: addrs_tx,
            x: Utc::now(),
        }
    }
}

impl Drop for AddrGuard {
    fn drop(&mut self) {
        // let known_addr = self.get_known_addr();

        // tdebug!(
        //     "p2p_discovery",
        //     "table",
        //     "Addr node (p2p endpoint: {}, known_at: {}, x: {}] is \
        //         pushed back to the queue",
        //     known_addr.p2p_endpoint(),
        //     known_addr.known_at,
        //     self.x,
        // );

        println!("addr guard pushed back, x: {}", self.x);

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
