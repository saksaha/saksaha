use p2p_identity::addr::Addr;

// pub(crate) enum Node {
//     val(EmptyNode)
//     addr(Addr),
//     // KnownNode(KnownNode),
//     // UnknownNode(UnknownNode),
// }

#[derive(Debug)]
pub(crate) enum Node {
    Empty,

    Valued(Addr),
}

// pub(crate) struct KnownNode {
//     peer: KnownPeer,
//     fail_count: u16,
// }

// pub(crate) struct UnknownNode {
//     peer: UnknownPeer,
//     fail_count: u16,
// }
