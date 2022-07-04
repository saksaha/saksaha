use super::AddrStatus;
use sak_crypto::{PublicKey, Signature};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct KnownAddr {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: u16,
    pub sig: Signature,
    pub public_key_str: String,
    pub public_key: PublicKey,
    pub status: RwLock<AddrStatus>,
}

impl KnownAddr {
    pub fn get_disc_endpoint(&self) -> String {
        sak_utils_net::make_endpoint(&self.ip, self.disc_port)
    }

    pub fn get_p2p_endpoint(&self) -> String {
        sak_utils_net::make_endpoint(&self.ip, self.p2p_port)
    }

    pub fn get_public_ket_short(&self) -> &str {
        &self.public_key_str[..6]
    }
}

impl std::fmt::Display for KnownAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addr_status = match self.status.try_read() {
            Ok(s) => format!("{:?}", s),
            Err(_) => "is being used".to_string(),
        };

        write!(
            f,
            "ip: {}, disc_port: {}, p2p_port: {:?}, status: {}",
            self.ip, self.disc_port, self.p2p_port, addr_status,
        )
    }
}
