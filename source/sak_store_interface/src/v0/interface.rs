use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type MRSAccessor = Box<dyn MRSInterface + Send + Sync>;

pub type StoreInterfaceError = Box<dyn std::error::Error + Send + Sync>;

pub trait MRSInterface {
    fn get_mrs_data(&self, key: &String) -> Result<Option<String>, StoreInterfaceError>;
    fn put_mrs_data(&self, key: &String, value: &String) -> Result<(), StoreInterfaceError>;
}

pub trait LedgerInterface {
    fn get_ctr_state(&self) -> Result<Option<Vec<u8>>, StoreInterfaceError>;
}

#[derive(Serialize, Deserialize)]
pub struct PreflightResponse {
    pub request_id: usize,
    pub data: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub attribute: HashMap<usize, usize>,
}
