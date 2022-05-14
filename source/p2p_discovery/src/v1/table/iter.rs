use super::AddrNode;
use chrono::Utc;
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};
use logger::terr;
use p2p_identity::addr::{KnownAddr, KnownAddrStatus};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;

pub struct AddrsIterator {
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<AddrNode>>>>,
    known_addrs_rx: Arc<RwLock<UnboundedReceiver<Arc<RwLock<AddrNode>>>>>,
}

pub struct AddrGuard {
    node: Arc<RwLock<AddrNode>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<AddrNode>>>>,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<AddrNode>>>>,
        known_addrs_rx: Arc<RwLock<UnboundedReceiver<Arc<RwLock<AddrNode>>>>>,
    ) -> AddrsIterator {
        AddrsIterator {
            known_addrs_tx,
            known_addrs_rx,
        }
    }

    pub async fn next(&self) -> Result<AddrGuard, String> {
        let mut rx = self.known_addrs_rx.write().await;

        match rx.recv().await {
            Some(n) => {
                let node = n.read().await;
                match &*node {
                    AddrNode::Known(_) => {
                        let addr_guard = AddrGuard {
                            known_addrs_tx: self.known_addrs_tx.clone(),
                            node: n.clone(),
                        };

                        return Ok(addr_guard);
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
    pub async fn get_known_addr(&self) -> Result<KnownAddr, String> {
        let node = self.node.read().await;

        match &*node {
            AddrNode::Known(n) => Ok(n.clone()),
            AddrNode::Unknown(n) => {
                return Err(format!(
                    "Unknown addr, which is invalid. disc_endpoint: {}",
                    n.disc_endpoint(),
                ))
            }
            AddrNode::Empty => return Err(format!("Addr node is empty")),
        }
    }
}

impl Drop for AddrGuard {
    fn drop(&mut self) {
        match self.known_addrs_tx.send(self.node.clone()) {
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

impl AddrGuard {
    pub fn new_dummy(
        public_key: k256::PublicKey,
        public_key_str: String,
        sig: Signature,
        disc_port: u16,
        p2p_port: u16,
    ) -> AddrGuard {
        let node = AddrNode::Known(KnownAddr {
            ip: "0.0.0.0".to_string(),
            disc_port,
            p2p_port,
            sig,
            public_key_str,
            known_at: Utc::now(),
            public_key,
            status: KnownAddrStatus::Initialized,
        });

        let (addrs_tx, _) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), Arc::new(RwLock::new(rx)))
        };

        AddrGuard {
            node: Arc::new(RwLock::new(node)),
            known_addrs_tx: addrs_tx,
        }
    }
}
