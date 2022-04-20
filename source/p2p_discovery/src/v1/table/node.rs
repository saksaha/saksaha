use p2p_identity::peer::{KnownPeer, UnknownPeer};

pub(crate) enum Node {
    KnownNode(KnownNode),
    UnknownNode(UnknownNode),
}

pub(crate) struct KnownNode {
    peer: KnownPeer,
    fail_count: u16,
}

pub(crate) struct UnknownNode {
    peer: UnknownPeer,
    fail_count: u16,
}
