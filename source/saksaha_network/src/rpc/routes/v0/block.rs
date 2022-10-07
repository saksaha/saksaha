use crate::system::SystemHandle;
use hyper::{Body, Response};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed, require_some_params, Params,
    RouteState,
};
use sak_types::Block;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockRequest {
    pub block_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockResponse {
    pub block: Option<Block>,
}

pub(in crate::rpc) async fn get_block(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "get_block should contain params",);

    let rb: GetBlockRequest = require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .ledger
        .dist_ledger
        .get_block(&rb.block_hash)
    {
        Ok(block) => {
            let get_block_resp = GetBlockResponse { block };

            return make_success_response(route_state, get_block_resp);
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockListRequest {
    pub offset: Option<u128>,
    pub limit: Option<u128>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockListResponse {
    pub block_list: Vec<Block>,
}

pub(in crate::rpc) async fn get_block_list(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "get_block_list should contain params",);

    let rb: GetBlockListRequest = require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .ledger
        .dist_ledger
        .get_block_list(rb.offset, rb.limit)
        .await
    {
        Ok(block_list) => {
            let get_block_resp = GetBlockListResponse { block_list };

            return make_success_response(route_state, get_block_resp);
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}
