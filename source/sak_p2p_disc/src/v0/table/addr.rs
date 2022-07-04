use super::SlotGuard;
use sak_p2p_addr::AddrStatus;
use sak_p2p_addr::KnownAddr;

pub struct DiscAddr {
    pub known_addr: KnownAddr,
    pub(crate) _addr_slot_guard: SlotGuard,
}

impl DiscAddr {
    pub fn get_public_key_short(&self) -> &str {
        &self.known_addr.public_key_str[..6]
    }
}
impl std::fmt::Display for DiscAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Addr - Known ({}])", self.known_addr)
    }
}

pub mod for_test {
    use super::*;
    use crate::Slot;
    use chrono::Utc;
    use sak_crypto::{PublicKey, Signature};
    use sak_p2p_addr::{AddrStatus, KnownAddr};
    use std::sync::Arc;
    use tokio::sync::{mpsc, RwLock};

    impl DiscAddr {
        pub fn new_dummy(
            public_key: PublicKey,
            public_key_str: String,
            sig: Signature,
            disc_port: u16,
            p2p_port: u16,
        ) -> DiscAddr {
            let (slots_tx, _rx) = mpsc::unbounded_channel();

            let addr = DiscAddr {
                known_addr: KnownAddr {
                    ip: "127.0.0.1".to_string(),
                    disc_port,
                    p2p_port,
                    sig,
                    public_key_str,
                    status: RwLock::new(AddrStatus::WhoAreYouSuccess {
                        at: Utc::now(),
                    }),
                    public_key,
                },
                _addr_slot_guard: SlotGuard {
                    _slot: Arc::new(Slot { _idx: 0 }),
                    slots_tx: Arc::new(slots_tx),
                },
            };

            addr
        }
    }
}
