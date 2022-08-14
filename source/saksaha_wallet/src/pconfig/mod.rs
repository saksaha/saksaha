use crate::WalletError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PConfig {
    public_key: String,
    secret: String,
}

impl PConfig {
    pub fn load() -> Result<PConfig, WalletError> {
        let c = PConfig {
            public_key: "dummy".to_string(),
            secret: "dummy".to_string(),
        };

        Ok(c)
    }
}
