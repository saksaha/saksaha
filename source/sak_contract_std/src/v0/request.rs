use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CtrCallType {
    Query,
    Execute,
}

pub type RequestArgs = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CtrRequest {
    pub req_type: String,
    pub args: RequestArgs,
    pub ctr_call_type: CtrCallType,
}

impl CtrRequest {
    pub fn parse(data: &[u8]) -> Result<CtrRequest, String> {
        match serde_json::from_slice(data) {
            Ok(o) => Ok(o),
            Err(err) => {
                return Err(format!("Error parsing request, err: {}", err));
            }
        }
    }
}
