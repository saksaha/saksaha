use crate::connection::Connection;
use crypto::{Secp256k1, SharedSecret};

pub struct Transport {
    pub conn: Connection,
    pub shared_secret: SharedSecret<Secp256k1>,
}

impl Transport {}
