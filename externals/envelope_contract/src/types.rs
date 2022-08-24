use sak_contract_std::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PublicKey = String;
pub type ChannelId = String;
pub type Date = String;
pub type EncryptedChatMessage = String;
pub type EncryptedEphSecret = String;

pub mod request_type {
    pub const OPEN_CH: &'static str = "open_ch";

    pub const SEND_MSG: &'static str = "send_msg";

    pub const GET_CH_LIST: &'static str = "get_ch_list";

    pub const GET_MSG: &'static str = "get_msgs";

    pub const GET_BALANCE: &'static str = "get_balance";
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}

impl Channel {
    pub fn new(
        ch_id: String,
        eph_key: String,
        sig: String,
    ) -> Result<Channel, ContractError> {
        let open_ch = Channel {
            ch_id,
            eph_key,
            sig,
        };

        Ok(open_ch)
    }

    pub fn default() -> Channel {
        Channel {
            ch_id: String::default(),
            eph_key: String::default(),
            sig: String::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub date: Date,
    pub user: PublicKey,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChListParams {
    pub dst_pk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenChParams {
    pub dst_pk: String,
    pub open_ch: Channel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMsgParams {
    pub ch_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMsgParams {
    pub ch_id: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvelopeStorage {
    pub open_ch_reqs: HashMap<PublicKey, Vec<Channel>>,
    pub chats: HashMap<ChannelId, Vec<EncryptedChatMessage>>,
}
