use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CtrCallType {
    Query,
    Execute,
}

pub type RequestArgs = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CtrRequest {
    pub ctr_addr: String,
    pub req_type: String,
    pub args: RequestArgs,
    pub ctr_call_type: CtrCallType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CtrRequestData {
    pub req_type: String,
    pub args: RequestArgs,
    pub ctr_call_type: CtrCallType,
}

impl CtrRequest {
    pub fn parse(ctr_addr: &String, data: &[u8]) -> Result<CtrRequest, String> {
        let data: CtrRequestData = match serde_json::from_slice(data) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("Error parsing request, err: {}", err));
            }
        };

        let req = CtrRequest {
            ctr_addr: ctr_addr.to_string(),
            req_type: data.req_type,
            args: data.args,
            ctr_call_type: data.ctr_call_type,
        };

        Ok(req)
    }
}
