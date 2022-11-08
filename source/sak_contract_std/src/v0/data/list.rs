use crate::{get_ctr_state_from_host, get_mrs_data_from_host, HostStorage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct List<T>
where
    T: AsRef<[u8]>,
{
    _name: String,
    _host_storage: HostStorage,
    data: HashMap<String, Vec<u8>>,
    phantom: Vec<T>,
}

impl<T> List<T>
where
    T: AsRef<[u8]>,
{
    pub fn new(_name: String, _host_storage: HostStorage) -> Self {
        List {
            _name,
            _host_storage,
            data: HashMap::new(),
            phantom: Vec::new(),
        }
    }

    pub fn init<B>(&mut self, data: B)
    where
        B: IntoIterator<Item = T>,
    {
        for (idx, d) in data.into_iter().enumerate() {
            let key: String = format!("{}_{}", self._name, idx);

            let val = d.as_ref().to_vec();

            self.data.insert(key, val);
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

    pub fn push(&mut self, value: Vec<u8>) {
        //TO-DO: get latest idx of the stored List and update index
        let latest_idx_key = String::from("latest_idx");

        let latest_idx = get_mrs_data_from_host(&latest_idx_key);

        let latest_idx = String::from_utf8(latest_idx).unwrap_or(String::from("0"));

        let key: String = format!("{}_{}", self._name, latest_idx);

        self.data.insert(key, value);
    }

    pub fn get_receipt(&self) -> HashMap<String, Vec<u8>> {
        self.data.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
