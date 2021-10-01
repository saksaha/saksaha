pub mod peer_store;

#[derive(Debug)]
pub enum PeerStatus {
    NotInitialized,

    Discovered,

}

#[derive(Debug)]
pub struct Peer {
    pub endpoint: String,
    pub peer_id: String,
    pub status: PeerStatus,
}

impl Peer {
    pub fn new(endpoint: String, peer_id: String, status: PeerStatus) -> Peer {
        Peer {
            endpoint,
            peer_id,
            status,
        }
    }
}
