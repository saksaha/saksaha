use crate::{get_ctr_state_from_host, get_mrs_data_from_host, HostStorage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dict<T> {
    _name: String,
    _host_storage: HostStorage,
    data: HashMap<String, Vec<u8>>,
    phantom: Vec<T>,
}

impl<T> Dict<T> {
    pub fn new(_name: String, _host_storage: HostStorage) -> Self {
        Dict {
            _name,
            _host_storage,
            data: HashMap::<String, Vec<u8>>::new(),
            phantom: Vec::new(),
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = match self._host_storage {
            HostStorage::MRS => get_mrs_data_from_host(&key),
            HostStorage::CtrState => get_ctr_state_from_host(&key),
        };

        data
    }

    pub fn push(&mut self, key: String, value: Vec<u8>) {
        let key = format!("{}_{}", self._name, key);

        self.data.insert(key, value);
    }

    pub fn get_receipt(&self) -> HashMap<String, Vec<u8>> {
        // let res = serde_json::to_vec(&self.receipt)?;

        // Ok(res)

        HashMap::from([("str_1".to_string(), vec![123])])
    }
}
