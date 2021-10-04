pub mod peer_store;

#[derive(Debug)]
pub enum Status {
    NotInitialized,

    Discovered,

}

#[derive(Debug)]
pub struct Peer {
    pub endpoint: String,
    pub peer_id: String,
    pub status: Status,
}

impl Peer {
    pub fn new(endpoint: String, peer_id: String, status: Status) -> Peer {
        Peer {
            endpoint,
            peer_id,
            status,
        }
    }
}
