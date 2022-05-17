use crate::AddrGuard;

use super::Addr;
use super::AddrVal;
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
use p2p_identity::addr::{AddrStatus, KnownAddr};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLockReadGuard,
};

pub struct AddrsIterator {
    addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
    known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
}

impl AddrsIterator {
    pub(crate) fn init(
        addr_recycle_tx: Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
        known_addrs_rx: Arc<RwLock<Receiver<Arc<RwLock<Addr>>>>>,
    ) -> AddrsIterator {
        AddrsIterator {
            addr_recycle_tx,
            known_addrs_rx,
        }
    }

    pub async fn next(&self) -> Result<AddrGuard, String> {
        let mut rx = self.known_addrs_rx.write().await;

        match rx.recv().await {
            Some(addr) => {
                let addr_guard = AddrGuard {
                    __internal_addr_recycle_tx: self.addr_recycle_tx.clone(),
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
