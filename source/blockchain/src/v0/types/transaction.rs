use super::Hashable;
use crypto::sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub created_at: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub pi: String,
    pub signature: String,
}

impl Hashable for Transaction {
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
