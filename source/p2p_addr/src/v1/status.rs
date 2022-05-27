use chrono::{DateTime, Utc};
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};

#[derive(Debug, Clone)]
pub enum AddrStatus {
    Invalid { err: String },
    Initialized,
    WhoAreYouInit { at: DateTime<Utc> },
    WhoAreYouSynRecv { at: DateTime<Utc> },
    WhoAreYouSuccess { at: DateTime<Utc> },
}

impl Default for AddrStatus {
    fn default() -> Self {
        AddrStatus::Initialized
    }
}
