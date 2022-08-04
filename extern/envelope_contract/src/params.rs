use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetMsgParams {
    pub ch_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetChListParams {
    pub dst_pk: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenChParams {
    pub dst_pk: String,
    pub input_serialized: Vec<u8>,
}
