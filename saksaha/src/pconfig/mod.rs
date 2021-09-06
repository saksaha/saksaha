pub mod path;
pub mod temp;

pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

pub struct PersistedP2PConfig {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl PConfig {
    pub fn new() -> Self {
        PConfig {
            p2p: PersistedP2PConfig {
                private_key: None,
                public_key: None,
            }
        }
    }
}
