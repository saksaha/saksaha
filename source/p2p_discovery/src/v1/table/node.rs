use p2p_identity::addr::{KnownAddr, UnknownAddr};

#[derive(Debug)]
pub(crate) enum AddrNode {
    Empty,
    Known(KnownAddr),
    Unknown(UnknownAddr),
}

impl AddrNode {
    pub fn is_empty(&self) -> bool {
        match self {
            AddrNode::Empty => true,
            _ => false,
        }
    }
}
