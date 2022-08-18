mod channel;
mod channel_list;
mod chat_message;
mod envelope_storage;
mod params;

pub use channel::*;
pub use channel_list::*;
pub use chat_message::*;
pub use envelope_storage::*;
pub use params::*;

pub type PublicKey = String;
pub type ChannelId = String;
pub type Date = String;
pub type EncryptedChatMessage = String;

pub type EnvelopeTypeError = Box<dyn std::error::Error + Send + Sync>;

pub mod request_type {
    pub const OPEN_CH: &'static str = "open_ch";

    pub const SEND_MSG: &'static str = "send_msg";

    pub const GET_CH_LIST: &'static str = "get_ch_list";

    pub const GET_MSG: &'static str = "get_msgs";
}
