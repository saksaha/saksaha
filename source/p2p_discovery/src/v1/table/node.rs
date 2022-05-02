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
    WhoAreYouSyncSent,
}

#[derive(Debug)]
pub(crate) struct NodeValue {
    addr: Addr,
    status: NodeStatus,
}
