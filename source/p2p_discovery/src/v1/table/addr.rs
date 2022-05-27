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
