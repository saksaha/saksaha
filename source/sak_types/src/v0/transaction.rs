use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    //
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    //
    pi: String,

    //
    signature: String,

    //
    #[serde(with = "serde_bytes")]
    contract_addr: Vec<u8>,

    // auto-generated value
    hash: String,
}

pub enum TxType {
    ContractCall,
    ContractDeploy,
    Others,
}

impl Transaction {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        pi: String,
        signature: String,
        contract_addr: Vec<u8>,
    ) -> Transaction {
        let hash = sak_crypto::compute_hash(&[
            created_at.as_bytes(),
            data.as_slice(),
            pi.as_bytes(),
            signature.as_bytes(),
            contract_addr.as_slice(),
        ]);

        Transaction {
            created_at,
            data,
            pi,
            signature,
            contract_addr,
            hash,
        }
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_pi(&self) -> &String {
        &self.pi
    }

    pub fn get_signature(&self) -> &String {
        &self.signature
    }

    pub fn get_contract_addr(&self) -> &Vec<u8> {
        &self.contract_addr
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    // fn __now_unused_get_tx_type(&self) -> TxType {
    //     let has_contract = self.contract_addr.len() > 0;

    //     if has_contract {
    //         return TxType::ContractCall;
    //     }

    //     TxType::Others
    // }
}
