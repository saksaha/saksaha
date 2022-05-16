use super::{slot::Slot, SlotGuard};
use p2p_identity::addr::{KnownAddr, UnknownAddr};
use std::sync::Arc;

pub(crate) enum Addr {
    Known(KnownAddrNode),
    Unknown(UnknownAddrNode),
}

pub struct KnownAddrNode {
    pub(crate) known_addr: KnownAddr,
    pub(crate) __internal_slot: SlotGuard,
}

impl KnownAddrNode {
    pub(crate) fn downgrade(&mut self) {
        println!("111");
    }
}

pub struct UnknownAddrNode {
    pub(crate) unknown_addr: UnknownAddr,
    __internal_slot: SlotGuard,
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Addr::Known(k) => {
                write!(f, "Addr - Known ({}])", k.known_addr)
            }
            Addr::Unknown(u) => {
                write!(f, "Addr - Unknown ({}])", u.unknown_addr)
            }
        }
    }
}

// #[derive(Debug)]
// pub(crate) enum AddrNode {
//     Empty,
//     Known(KnownAddr),
//     Unknown(UnknownAddr),
// }

// impl AddrNode {
//     pub fn is_empty(&self) -> bool {
//         match self {
//             AddrNode::Empty => true,
//             _ => false,
//         }
//     }
// }
