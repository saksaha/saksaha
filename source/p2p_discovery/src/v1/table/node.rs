use p2p_identity::addr::Addr;

#[derive(Debug)]
pub(crate) enum NodeStatus {
    Initialized,
    WhoAreYouSynSent,
    WhoAreYouAckSent,
    WhoAreYouAckRecvd,
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
