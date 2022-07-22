use super::utils;
use crate::rpc::{router::HeaderFactory, RPCError};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

pub(in crate::rpc) const JSON_RPC_VERSION: &'static str = "2.0";

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct JsonResponse<R: Serialize> {
    pub jsonrpc: String,
    pub error: Option<HandleError>,
    pub result: Option<R>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct JsonRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Vec<u8>>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct HandleError {
    pub msg: String,
}
