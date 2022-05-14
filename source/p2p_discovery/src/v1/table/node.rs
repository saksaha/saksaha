use p2p_identity::addr::{KnownAddr, UnknownAddr};

// #[derive(Debug)]
// pub(crate) enum NodeStatus {
//     Initialized,
//     WhoAreYouInit,
//     WhoAreYouRecv,
//     HandshakeSynFail { fail_count: usize },
// }

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

// #[derive(Debug)]
// pub(crate) struct KnownAddrNode {
//     pub(crate) addr: KnownAddr,
//     // pub(crate) status: NodeStatus,
// }

// #[derive(Debug)]
// pub(crate) struct UnknownAddrNode {
//     pub(crate) addr: UnknownAddr,
//     // pub(crate) status: NodeStatus,
// }
