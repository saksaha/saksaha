mod tx;

pub use tx::*;

use serde::{Deserialize, Serialize};

pub const JSON_RPC_2: &'static str = "2.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse<R: Serialize> {
    pub jsonrpc: String,
    pub error: Option<JsonRPCError>,
    pub result: Option<R>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Vec<u8>>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRPCError {
    pub msg: String,
}
