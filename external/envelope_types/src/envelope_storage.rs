use crate::{Channel, ChannelId, EncryptedChatMessage, PublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvelopeStorage {
    pub open_ch_reqs: HashMap<PublicKey, Vec<Channel>>,
    pub chats: HashMap<ChannelId, Vec<EncryptedChatMessage>>,
}
