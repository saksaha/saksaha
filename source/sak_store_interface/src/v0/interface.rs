use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MRSInterfaceError = Box<dyn std::error::Error + Send + Sync>;

pub type LedgerInterfaceError = Box<dyn std::error::Error + Send + Sync>;

pub type MRSAccessor = Box<dyn MRSInterface + Send + Sync>;

pub type LedgerAccessor = Box<dyn LedgerInterface + Send + Sync>;

pub trait MRSInterface {
    fn get_mrs_data(&self, key: &String) -> Result<Option<String>, MRSInterfaceError>;

    fn put_mrs_data(&self, key: &String, value: &String) -> Result<(), MRSInterfaceError>;

    // async fn get_session(&self, session_id: String) -> Session;
    fn add_session(&self, session: Session);
}

pub trait LedgerInterface {
    fn get_ctr_state(&self) -> Result<Option<Vec<u8>>, LedgerInterfaceError>;
}

#[derive(Serialize, Deserialize)]
pub struct PreflightResponse {
    pub request_id: usize,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub receipt: HashMap<String, Vec<u8>>,
}
