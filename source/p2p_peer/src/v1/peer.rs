use p2p_transport::transport::Transport;

// #[derive(Debug, PartialEq)]
// pub enum Status {
//     Empty,
//     NotInitialized,
//     DiscoverySuccess,
//     HandshakeSuccess,
//     HandshakeFail(String),
// }

pub struct Peer {
    pub transport: Transport,
}
