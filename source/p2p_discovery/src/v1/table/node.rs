use p2p_identity::addr::Addr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) enum Node {
    Empty,

    Valued(Arc<Mutex<NodeValue>>),
}

#[derive(Debug)]
pub(crate) enum NodeStatus {
    Initialized,
    WhoAreYouSynSent,
}

#[derive(Debug)]
pub(crate) struct NodeValue {
    pub(crate) addr: Addr,
    pub(crate) status: NodeStatus,
}

impl NodeValue {
    pub(crate) fn new(addr: Addr) -> NodeValue {
        NodeValue {
            addr,
            status: NodeStatus::Initialized,
        }
    }
}
