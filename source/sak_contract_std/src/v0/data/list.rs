use crate::{get_mrs_data_from_host, put_mrs_data_to_host};
use std::collections::HashMap;

#[derive(Debug)]
pub struct List<T> {
    _name: String,
    receipt: HashMap<String, Vec<u8>>,
    phantom: Vec<T>,
}

impl<T> List<T> {
    pub fn new(_name: String) -> Self {
        List {
            _name,
            receipt: HashMap::new(),
            phantom: Vec::new(),
        }
    }

    pub fn init<B>(&self, data: B)
    where
        B: IntoIterator<Item = T>,
    {
        for (idx, d) in data.into_iter().enumerate() {
            let key: String = format!("{}_{}", self._name, idx);

            self.receipt.insert(key, d);
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }

    pub fn put(&self, value: &String) {
        let key: String = format!("{}", self._name);

        put_mrs_data_to_host(&key, value);
    }

    pub fn push(&mut self, value: Vec<u8>) {
        //TO-DO: get latest idx of the stored List and update index
        let latest_idx_key = String::from("latest_idx");
        let latest_idx = get_mrs_data_from_host(&latest_idx_key);

        let latest_idx = 0;

        let key: String = format!("{}_{}", self._name, latest_idx);

        self.receipt.insert(key, value);
    }

    pub fn receipt(&self) -> HashMap<String, Vec<u8>> {
        self.receipt.clone()
    }
}
