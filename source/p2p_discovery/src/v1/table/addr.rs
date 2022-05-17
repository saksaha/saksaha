use super::SlotGuard;
use p2p_identity::addr::{AddrStatus, KnownAddr, UnknownAddr};

pub struct Addr {
    pub val: AddrVal,
    pub(crate) __internal_slot: SlotGuard,
}

pub enum AddrVal {
    Known(KnownAddr),
    Unknown(UnknownAddr),
}

impl Addr {
    pub fn get_status(&self) -> &AddrStatus {
        match &self.val {
            AddrVal::Unknown(u) => &u.status,
            AddrVal::Known(k) => &k.status,
        }
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.val {
            AddrVal::Known(k) => {
                write!(f, "Addr - Known ({}])", k)
            }
            AddrVal::Unknown(u) => {
                write!(f, "Addr - Unknown ({}])", u)
            }
        }
    }
}
