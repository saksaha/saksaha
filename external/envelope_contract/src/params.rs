use crate::OpenCh;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChListParams {
    pub dst_pk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenChParams {
    // pub input_serialized: Vec<u8>,
    pub dst_pk: String,
    pub open_ch: OpenCh,
    // pub ch_id: String,
    // pub eph_pk: String,
    // pub sig: String,
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
