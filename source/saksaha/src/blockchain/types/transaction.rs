use std::time::SystemTime;

use super::{Hash, Hashable};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Transaction {
    pub(crate) created_at: String,
    pub(crate) data: Vec<u8>,
    pub(crate) pi: String,
    pub(crate) signature: String,
}

impl Hashable for Transaction {
    fn get_hash(&self) -> Result<Hash, String> {
        let hash = {
            let mut h = Sha3_256::new();
            let v = match serde_json::to_value(&self) {
                Ok(v) => v,
                Err(err) => {
                    return Err(format!(
                        "Failed to serialize self, err: {}",
                        err
                    ))
                }
            };
            h.update(v.to_string());
            h.finalize()
        };

        Ok(Hash {
            hash: format!("{:x}", hash),
        })
    }
}
