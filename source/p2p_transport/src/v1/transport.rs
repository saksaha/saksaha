use crypto::{SharedSecret, Secp256k1};
use tokio::net::TcpStream;

pub struct Transport {
    // pub ip: String,
    // pub port: u16,
    pub stream: TcpStream,
    pub shared_secret: SharedSecret<Secp256k1>,
}

impl Transport {

}
