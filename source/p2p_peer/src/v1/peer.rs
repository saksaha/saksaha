use p2p_discovery::AddrGuard;
use p2p_transport::transport::Transport;


pub struct Peer {
    pub p2p_port: u16,
    pub public_key_str: String,
    pub transport: Transport,
    pub addr_guard: Option<AddrGuard>,
    pub status: PeerStatus,
}

pub enum PeerStatus {
    Initialized,
    HandshakeInitSuccess,
    HandshakeRecvSuccess,
    HandshakeInitFail { err: String },
    HandshakeRecvFail { err: String },
}

impl std::fmt::Display for PeerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerStatus::Initialized => {
                write!(f, "Initialized")
            }
            PeerStatus::HandshakeInitSuccess => {
                write!(f, "HandshakeInitSuccess")
            }
            PeerStatus::HandshakeRecvSuccess => {
                write!(f, "HandshakeRecvSuccess")
            }
            PeerStatus::HandshakeInitFail { err } => {
                write!(f, "HandshakeInitFail, err: {}", err)
            }
            PeerStatus::HandshakeRecvFail { err } => {
                write!(f, "HandshakeRecvFail, err: {}", err)
            }
        }
    }
}

// impl std::fmt::Display for Peer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "PeerNode (peer: {}, status: {})", self.peer, self.status)
//     }
// }

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let socket_addr = &self.transport.conn.socket_addr;
        let public_key_str = &self.public_key_str;

        write!(
            f,
            "Peer (socket_addr: {}, public_key_str: {}, status: {})",
            socket_addr, public_key_str, self.status,
        )
    }
}
