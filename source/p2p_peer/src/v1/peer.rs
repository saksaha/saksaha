use p2p_transport::transport::Transport;

pub struct Peer {
    pub transport: Transport,
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let socket_addr = &self.transport.conn.socket_addr;
        let public_key_str = &self.transport.public_key_str;

        write!(
            f,
            "Peer (socket_addr: {}, public_key_str: {})",
            socket_addr, public_key_str
        )
    }
}
