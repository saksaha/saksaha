use super::Hashable;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Block {
    pub(crate) transactions: Vec<String>,
    pub(crate) signatures: Vec<String>,
    pub(crate) created_at: String,
    pub(crate) height: String,
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
