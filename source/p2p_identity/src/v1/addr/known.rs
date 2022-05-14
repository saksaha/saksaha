use chrono::{DateTime, Utc};
use crypto::PublicKey;
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};

#[derive(Debug, Clone)]
pub struct KnownAddr {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: u16,
    pub sig: Signature,
    pub public_key_str: String,
    pub public_key: PublicKey,
    pub known_at: DateTime<Utc>,
    pub status: KnownAddrStatus,
}

#[derive(Debug, Clone)]
pub enum KnownAddrStatus {
    Initialized,
    WhoAreYouInit,
    WhoAreYouRecv,
    WhoAreYouAckRecv,
    HandshakeSynFail { fail_count: usize },
}

impl KnownAddr {
    pub fn disc_endpoint(&self) -> String {
        super::make_endpoint(&self.ip, self.disc_port)
    }

    pub fn p2p_endpoint(&self) -> String {
        super::make_endpoint(&self.ip, self.p2p_port)
    }
}

impl std::fmt::Display for KnownAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ip: {}, disc_port: {}, p2p_port: {:?}",
            self.ip, self.disc_port, self.p2p_port,
        )
    }
}
