use crate::{get_mrs_data_from_host, ContractError, RET_LEN_SIZE};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dict {
    _name: String,
    receipt: HashMap<String, Vec<u8>>,
}

impl Dict {
    pub fn new(_name: String) -> Dict {
        Dict {
            _name,
            receipt: HashMap::<String, Vec<u8>>::new(),
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }

    pub fn push(&mut self, key: String, value: Vec<u8>) {
        self.receipt.insert(key, value);
    }

    pub fn receipt(&self) -> Result<Vec<u8>, ContractError> {
        let res = serde_json::to_vec(&self.receipt)?;

        Ok(res)
    }
}
