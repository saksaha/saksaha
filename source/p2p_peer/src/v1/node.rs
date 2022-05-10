use super::peer::Peer;
use logger::terr;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

pub struct Node {
    pub value: NodeValue,
    pub status: NodeStatus,
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
    pub node_retrieval_tx: Arc<UnboundedSender<Arc<Mutex<Node>>>>,
}

impl Drop for NodeGuard {
    fn drop(&mut self) {
        match self.node_retrieval_tx.send(self.node.clone()) {
            Ok(_) => (),
            Err(err) => {
                terr!(
                    "p2p_peer",
                    "",
                    "Cannot retrieve peer node after use, err: {}",
                    err
                );
            }
        }
    }
}
