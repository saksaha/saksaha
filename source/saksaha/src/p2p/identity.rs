use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub secret: String,
    pub public_key: String,
}
