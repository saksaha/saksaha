use crate::get_mrs_data_from_host;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dict<T> {
    _name: String,
    receipt: HashMap<String, Vec<u8>>,
    phantom: Vec<T>,
}

impl<T> Dict<T> {
    pub fn new(_name: String) -> Self {
        Dict {
            _name,
            receipt: HashMap::<String, Vec<u8>>::new(),
            phantom: Vec::new(),
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }

    pub fn push(&mut self, key: String, value: Vec<u8>) {
        let key = format!("{}_{}", self._name, key);

        self.receipt.insert(key, value);
    }

    pub fn receipt(&self) -> HashMap<String, Vec<u8>> {
        // let res = serde_json::to_vec(&self.receipt)?;

        // Ok(res)

        HashMap::from([("str_1".to_string(), vec![123])])
    }
}
