use super::Addr;
// use crate::AddrGuard;
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};
use logger::terr;
use p2p_addr::AddrStatus;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{mpsc::Receiver, OwnedMutexGuard};
use tokio::sync::{mpsc::UnboundedSender, OwnedRwLockWriteGuard};

pub struct AddrsIterator {
    addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
    known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
    _addrs_it_lock: OwnedMutexGuard<usize>,
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
            _addrs_it_lock: addrs_it_lock,
        }
    }

    // Returning newly "discovered" addresses
    pub async fn next(&self) -> Result<Arc<RwLock<Addr>>, String> {
        let mut known_addrs_rx_lock = self.known_addrs_rx.write().await;

        loop {
            match known_addrs_rx_lock.recv().await {
                Some(a) => {
                    // let addr_guard = AddrGuard {
                    //     addr_recycle_tx: self.addr_recycle_tx.clone(),
                    //     addr,
                    // };

                    // return Ok(addr_guard);

                    let addr = a.clone();
                    let addr_lock = a.write_owned().await;
                    let addr_status = addr_lock.get_status();

                    match addr_status {
                        AddrStatus::WhoAreYouSuccess { .. } => {
                            drop(addr_lock);

                            return Ok(addr);
                        }
                        _ => (),
                    };
                }
                None => {
                    terr!(
                        "p2p_discovery",
                        "table",
                        "Known addrs queue has been closed. Coudn't retrieve \
                        known address.",
                    );

                    return Err(format!(
                        "Known addrs queue has been closed. Coudn't retrieve \
                        known address.",
                    ));
                }
            }
        }
    }
}
