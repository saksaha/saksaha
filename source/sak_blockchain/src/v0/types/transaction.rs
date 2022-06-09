use crate::Hashable;
use sak_crypto::sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub created_at: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub pi: String,
    pub signature: String,
    #[serde(with = "serde_bytes")]
    pub contract: Vec<u8>,
}

pub enum TxType {
    ContractCall,
    ContractDeploy,
    Others,
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
    pub fn get_tx_type(&self) -> TxType {
        let has_contract = self.contract.len() > 0;

        if has_contract {
            // data peek
            // return TxType::ContractDeploy;

            return TxType::ContractCall;
        }

        TxType::Others
    }
}
