use chrono::{DateTime, Duration, Local, Utc};
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

impl AddrStatus {
    pub fn is_registered_long_ago(&self, how_long: Duration) -> bool {
        if let AddrStatus::WhoAreYouSuccess { at } = self {
            let now = Local::now();
            return at.signed_duration_since(now) > how_long;
        }
        // .signed_duration_since(rhs)
        return false;
    }
}

impl Default for AddrStatus {
    fn default() -> Self {
        AddrStatus::Initialized
    }
}
