use p2p_identity::addr::Addr;
use std::sync::Arc;
use tokio::sync::Mutex;

// #[derive(Debug)]
// pub(crate) enum Node {
//     Empty,

//     Valued(NodeValue),
// }

#[derive(Debug)]
pub(crate) enum NodeStatus {
    Initialized,
    WhoAreYouSynSent,
    WhoAreYouAckSent,
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

impl NodeValueInner {
    pub(crate) fn new(addr: Addr) -> NodeValueInner {
        NodeValueInner {
            addr,
            status: NodeStatus::Initialized,
        }
    }
}
