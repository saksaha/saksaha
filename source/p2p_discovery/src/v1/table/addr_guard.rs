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
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::RwLock;

pub struct AddrGuard {
    pub addr: Arc<RwLock<Addr>>,
    pub(crate) __internal_addr_recycle_tx:
        Arc<UnboundedSender<Arc<RwLock<Addr>>>>,
}

impl Drop for AddrGuard {
    fn drop(&mut self) {
        match self.__internal_addr_recycle_tx.send(self.addr.clone()) {
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

pub mod testing {
    use super::*;

    impl AddrGuard {
        pub fn new_dummy(
            public_key: k256::PublicKey,
            public_key_str: String,
            sig: Signature,
            disc_port: u16,
            p2p_port: u16,
        ) -> AddrGuard {
            let (slots_tx, _rx) = mpsc::unbounded_channel();

            let addr = Addr {
                val: AddrVal::Known(KnownAddr {
                    ip: "0.0.0.0".to_string(),
                    disc_port,
                    p2p_port,
                    sig,
                    public_key_str,
                    status: AddrStatus::WhoAreYouSuccess { at: Utc::now() },
                    public_key,
                }),
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
                addr: Arc::new(RwLock::new(addr)),
                __internal_addr_recycle_tx: addrs_tx,
            }
        }
    }
}
