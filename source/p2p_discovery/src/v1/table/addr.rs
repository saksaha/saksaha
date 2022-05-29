use super::SlotGuard;
use p2p_addr::{AddrStatus, KnownAddr};

pub struct Addr {
    pub known_addr: KnownAddr,
    pub(crate) addr_slot_guard: SlotGuard,
}

impl Addr {
    pub fn get_status(&self) -> &AddrStatus {
        &self.known_addr.status
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Addr - Known ({}])", self.known_addr)
    }
}

pub mod for_test {
    use std::sync::Arc;

    use crate::Slot;

    use super::*;
    use chrono::Utc;
    use crypto::Signature;
    use k256::ecdsa::signature::Signature;
    use p2p_addr::{AddrStatus, KnownAddr};
    use tokio::sync::mpsc;

    impl Addr {
        pub fn new_dummy(
            public_key: k256::PublicKey,
            public_key_str: String,
            sig: Signature,
            disc_port: u16,
            p2p_port: u16,
        ) -> Addr {
            let (slots_tx, _rx) = mpsc::unbounded_channel();

            let addr = Addr {
                known_addr: KnownAddr {
                    ip: "0.0.0.0".to_string(),
                    disc_port,
                    p2p_port,
                    sig,
                    public_key_str,
                    status: AddrStatus::WhoAreYouSuccess { at: Utc::now() },
                    public_key,
                },
                addr_slot_guard: SlotGuard {
                    slot: Arc::new(Slot { idx: 0 }),
                    slots_tx: Arc::new(slots_tx),
                },
            };

            addr

            // let (addrs_tx, _) = {
            //     let (tx, rx) = mpsc::unbounded_channel();
            //     (Arc::new(tx), Arc::new(RwLock::new(rx)))
            // };

            // AddrGuard {
            //     addr: Arc::new(RwLock::new(addr)),
            //     addr_recycle_tx: addrs_tx,
            // }
        }
    }
}
