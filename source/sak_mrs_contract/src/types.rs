use serde::{Deserialize, Serialize};

pub type PublicKey = String;
pub type ChannelId = String;
pub type Date = String;
pub type EncryptedChatMessage = String;
pub type EncryptedEphSecret = String;

pub mod request_type {
    pub const RESERVE: &str = "reserve";
    pub const GET_SLOT: &str = "get_slot";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutableRecordStorage {
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    pub pk: PublicKey,
    pub timestamp: String,
    pub slot_number: usize,
}

impl Slot {
    pub fn default() -> Slot {
        Slot {
            pk: String::default(),
            timestamp: String::default(),
            slot_number: usize::default(),
        }
    }

    pub fn new(pk: PublicKey, timestamp: String, slot_number: usize) -> Slot {
        Slot {
            pk,
            timestamp,
            slot_number,
        }
    }
}
