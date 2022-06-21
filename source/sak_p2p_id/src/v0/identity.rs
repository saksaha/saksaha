use crate::Credential;
use std::sync::Arc;

pub struct Identity {
    pub credential: Arc<Credential>,
    pub p2p_port: u16,
    // pub disc_port: u16,
}
