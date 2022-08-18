use crate::Channel;
use serde::{Deserialize, Serialize};

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
