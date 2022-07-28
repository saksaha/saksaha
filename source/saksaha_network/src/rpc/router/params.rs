use crate::rpc::{router::HeaderFactory, RPCError};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use sak_rpc_interface::{JsonRPCError, JsonRequest, JsonResponse, JSON_RPC_2};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub(in crate::rpc) fn parse_params<P: DeserializeOwned>(
    params: &Vec<u8>,
) -> Result<P, RPCError> {
    match serde_json::from_slice(params) {
        Ok(r) => Ok(r),
        Err(err) => Err(format!("Cannot parse params, err: {}", err).into()),
    }
}
