use super::KnownAddrNode;
use super::Slot;
use super::SlotGuard;
use chrono::Utc;
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};
use logger::{tdebug, terr};
use p2p_identity::addr::{KnownAddr, KnownAddrStatus};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLockReadGuard,
};

pub struct AddrsIterator {
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<KnownAddrNode>>>>,
    known_addrs_rx: Arc<RwLock<UnboundedReceiver<Arc<RwLock<KnownAddrNode>>>>>,
}

pub struct AddrGuard {
    pub should_be_recycled: bool,
    pub known_addr_entry: Arc<RwLock<KnownAddrNode>>,
    known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<KnownAddrNode>>>>,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs_tx: Arc<UnboundedSender<Arc<RwLock<KnownAddrNode>>>>,
        known_addrs_rx: Arc<
            RwLock<UnboundedReceiver<Arc<RwLock<KnownAddrNode>>>>,
        >,
    ) -> AddrsIterator {
        AddrsIterator {
            known_addrs_tx,
            known_addrs_rx,
        }
    }

    pub async fn next(&self) -> Result<AddrGuard, String> {
        let mut rx = self.known_addrs_rx.write().await;

        match rx.recv().await {
            Some(known_addr_entry) => {
                let addr_guard = AddrGuard {
                    should_be_recycled: true,
                    known_addrs_tx: self.known_addrs_tx.clone(),
                    known_addr_entry,
                };

                return Ok(addr_guard);
                // AddrNode::Known(_) => {
                //     let addr_guard = AddrGuard {
                //         should_be_recycled: true,
                //         known_addrs_tx: self.known_addrs_tx.clone(),
                //         node: n.clone(),
                //     };

                //     return Ok(addr_guard);
                // }
                // _ => {
                //     return Err(format!(
                //         "Invalid address is popped out of known address \
                //         queue"
                //     ));
                // }
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
        let addr = self.known_addr_entry.read().await;

        Ok(addr.known_addr.clone())
    }
}

impl Drop for AddrGuard {
    fn drop(&mut self) {
        if self.should_be_recycled {
            match self.known_addrs_tx.send(self.known_addr_entry) {
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
        } else {
            tdebug!(
                "p2p_discovery",
                "table",
                "Addr guard is dropped. Most likely the same endpoint \
                is registered within a different addr guard"
            );
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
        let (slots_tx, rx) = mpsc::unbounded_channel();

        let addr = KnownAddrNode {
            known_addr: KnownAddr {
                ip: "0.0.0.0".to_string(),
                disc_port,
                p2p_port,
                sig,
                public_key_str,
                status: KnownAddrStatus::WhoAreYouSuccess { at: Utc::now() },
                public_key,
            },
            __internal_slot: SlotGuard {
                slot: Arc::new(Slot { idx: 0 }),
                slots_tx: Arc::new(slots_tx),
            },
        };

        let (addrs_tx, _) = {
            let (tx, rx) = mpsc::unbounded_channel();
            (Arc::new(tx), Arc::new(RwLock::new(rx)))
        };

        AddrGuard {
            should_be_recycled: true,
            known_addr_entry: Arc::new(RwLock::new(addr)),
            known_addrs_tx: addrs_tx,
        }
    }
}
