use crate::Channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}
