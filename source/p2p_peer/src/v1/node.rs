use super::peer::Peer;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

pub struct Node {
    pub value: NodeValue,
    pub status: NodeStatus,
    pub node_retrieval_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
}

impl Node {
    pub fn is_empty(&self) -> bool {
        if let NodeValue::Empty = self.value {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_used(&self) -> bool {
        if let NodeStatus::Used = self.status {
            return true;
        } else {
            return false;
        }
    }
}

pub enum NodeStatus {
    Available,
    Used,
}

pub enum NodeValue {
    Empty,

    Valued(Peer),
}

pub struct NodeGuard {
    pub node: Arc<Mutex<Node>>,
}

impl Drop for NodeGuard {
    fn drop(&mut self) {
        // self.node.status = NodeStatus::Initialized;
    }
}
