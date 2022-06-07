use super::Hashable;
use sak_crypto::sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub created_at: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub pi: String,
    pub signature: String,
    pub contract: Vec<u8>,
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

impl Transaction {
    fn has_contract(&self) -> Result<bool, String> {
        let is_has_contract =
            if self.contract.len() > 0 { true } else { false };
        Ok(is_has_contract)
    }
}
