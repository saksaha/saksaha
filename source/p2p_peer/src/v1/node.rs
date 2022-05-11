use super::peer::Peer;

pub enum Node {
    Empty,
    Peer(PeerNode),
}

pub struct PeerNode {
    pub peer: Peer,
    pub status: NodeStatus,
}

impl Node {
    pub fn is_empty(&self) -> bool {
        if let Node::Empty = &self {
            return true;
        } else {
            return false;
        }
    }
}

pub enum NodeStatus {
    Initialized,
    HandshakeInitSuccess,
    HandshakeRecvSuccess,
    HandshakeInitFail { err: String },
    HandshakeRecvFail { err: String },
}

impl std::fmt::Display for PeerNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PeerNode (peer: {}, status: {})", self.peer, self.status)
    }
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            NodeStatus::Initialized => {
                write!(f, "Initialized")
            }
            NodeStatus::HandshakeInitSuccess => {
                write!(f, "HandshakeInitSuccess")
            }
            NodeStatus::HandshakeRecvSuccess => {
                write!(f, "HandshakeRecvSuccess")
            }
            NodeStatus::HandshakeInitFail { err } => {
                write!(f, "HandshakeInitFail, err: {}", err)
            }
            NodeStatus::HandshakeRecvFail { err } => {
                write!(f, "HandshakeRecvFail, err: {}", err)
            }
        }
    }
}
