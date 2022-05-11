use p2p_identity::addr::Addr;

#[derive(Debug)]
pub(crate) enum NodeStatus {
    Initialized,
    WhoAreYouInit { fail_count: usize },
    WhoAreYouRecv { fail_count: usize },
    HandshakeSynFail { fail_count: usize },
}

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) value: NodeValue,
}

#[derive(Debug)]
pub(crate) enum NodeValue {
    Empty,
    Valued(NodeValueInner),
}

#[derive(Debug)]
pub(crate) struct NodeValueInner {
    pub(crate) addr: Addr,
    pub(crate) status: NodeStatus,
}
