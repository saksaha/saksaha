use super::SlotGuard;
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
