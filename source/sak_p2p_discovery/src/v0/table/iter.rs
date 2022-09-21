use super::DiscAddr;
use sak_logger::error;
use sak_p2p_addr::AddrStatus;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{mpsc::Receiver, OwnedMutexGuard};

pub struct AddrsIterator {
    known_addrs_rx: Arc<RwLock<Receiver<Arc<DiscAddr>>>>,
    _addrs_it_lock: OwnedMutexGuard<usize>,
}

impl AddrsIterator {
    pub(crate) fn init(
        known_addrs_rx: Arc<RwLock<Receiver<Arc<DiscAddr>>>>,
        addrs_it_lock: OwnedMutexGuard<usize>,
    ) -> AddrsIterator {
        AddrsIterator {
            known_addrs_rx,
            _addrs_it_lock: addrs_it_lock,
        }
    }

    // Returning newly "discovered" addresses
    pub async fn next(&self) -> Result<Arc<DiscAddr>, String> {
        let mut known_addrs_rx_lock = self.known_addrs_rx.write().await;

        loop {
            match known_addrs_rx_lock.recv().await {
                Some(addr) => {
                    let addr_status_lock = addr.known_addr.status.read().await;

                    match *addr_status_lock {
                        AddrStatus::WhoAreYouSuccess { .. } => {
                            drop(addr_status_lock);
                            return Ok(addr);
                        }
                        _ => (),
                    };
                }
                None => {
                    error!(
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
