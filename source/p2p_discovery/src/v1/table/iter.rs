use super::Addr;
use crate::AddrGuard;
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tokio::sync::{mpsc::Receiver, OwnedMutexGuard};

pub struct AddrsIterator {
    addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
    known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
    addrs_it_lock: OwnedMutexGuard<usize>,
}

impl AddrsIterator {
    pub(crate) fn init(
        addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
        known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
        addrs_it_lock: OwnedMutexGuard<usize>,
    ) -> AddrsIterator {
        AddrsIterator {
            addr_recycle_tx,
            known_addrs_rx,
            addrs_it_lock,
        }
    }

    pub async fn next(&self) -> Result<AddrGuard, String> {
        let mut known_addrs_rx_lock = self.known_addrs_rx.write().await;

        match known_addrs_rx_lock.recv().await {
            Some(addr) => {
                let addr_guard = AddrGuard {
                    addr_recycle_tx: self.addr_recycle_tx.clone(),
                    addr,
                };

                return Ok(addr_guard);
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
