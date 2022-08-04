use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum CtrCallType {
    Query,
    Execute,
}

// pub type RequestArgs = HashMap<String, String>;
pub type RequestArgs = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub req_type: String,
    // pub args: RequestArgs,
    pub args: Vec<u8>,
    pub ctr_call_type: CtrCallType,
}

impl Request {
    pub fn parse(data: &[u8]) -> Result<Request, String> {
        match serde_json::from_slice(data) {
            Ok(o) => Ok(o),
            Err(err) => {
                return Err(format!("Error parsing request, err: {}", err));
            }
        }
    }
}
