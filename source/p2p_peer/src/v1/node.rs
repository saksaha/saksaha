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
