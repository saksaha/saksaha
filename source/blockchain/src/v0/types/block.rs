use super::Hashable;
use crypto::sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub transactions: Vec<String>,
    pub signatures: Vec<String>,
    pub created_at: String,
    pub height: String,
}

impl Hashable for Block {
    fn get_hash(&self) -> Result<String, String> {
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

        let h = format!("{:x}", hash);

        Ok(h)
    }
}
