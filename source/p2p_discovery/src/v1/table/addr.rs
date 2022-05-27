use p2p_addr::{AddrStatus, KnownAddr, UnknownAddr};

use super::SlotGuard;

pub struct Addr {
    pub known_addr: KnownAddr,
    pub(crate) addr_slot_guard: SlotGuard,
}

// pub enum AddrVal {
//     Known(KnownAddr),
//     Unknown(UnknownAddr),
// }

impl Addr {
    pub fn get_status(&self) -> &AddrStatus {
        &self.known_addr.status
        // match &self.val {
        //     AddrVal::Unknown(u) => &u.status,
        //     AddrVal::Known(k) => &k.status,
        // }
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Addr - Known ({}])", self.known_addr)
        // match &self.val {
        //     AddrVal::Known(k) => {
        //         write!(f, "Addr - Known ({}])", k)
        //     }
        //     AddrVal::Unknown(u) => {
        //         write!(f, "Addr - Unknown ({}])", u)
        //     }
        // }
    }
}
