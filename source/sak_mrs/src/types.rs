use sak_contract_std::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PublicKey = String;
pub type ChannelId = String;
pub type Date = String;
pub type EncryptedChatMessage = String;
pub type EncryptedEphSecret = String;

pub mod request_type {
    pub const RESERVE: &str = "reserve";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutableRecordStorage {
    pub slots: HashMap<String, Slot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    pk: PublicKey,
    timestamp: String,
}

impl Slot {
    pub fn new() -> Slot {
        Slot {
            pk: String::default(),
            timestamp: String::default(),
        }
    }
}
