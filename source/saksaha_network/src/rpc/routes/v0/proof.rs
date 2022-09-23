use crate::system::SystemHandle;
use hyper::{Body, Response};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed, require_some_params, Params,
    RouteState,
};
use sak_types::{Cm, CmIdx};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetAuthPathRequest {
    pub cm_idx: CmIdx,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetAuthPathResponse {
    pub auth_path: Vec<(Cm, bool)>,
}

pub(in crate::rpc) async fn get_auth_path(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "get_auth_path should contain params",);

    let rb: GetAuthPathRequest = require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_auth_path(&rb.cm_idx)
        .await
    {
        Ok(auth_path) => {
            let get_auth_path_resp = GetAuthPathResponse { auth_path };

            return make_success_response(route_state, get_auth_path_resp);
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetCmIdxRequest {
    pub cm: Cm,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetCmIdxResponse {
    pub cm_idx: Option<CmIdx>,
}

pub(in crate::rpc) async fn get_cm_idx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "get_cm_idx should contain params",);

    let rb: GetCmIdxRequest = require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_cm_idx_by_cm(&rb.cm)
        .await
    {
        Ok(cm_idx) => {
            let get_auth_path_resp = GetCmIdxResponse { cm_idx };

            return make_success_response(route_state, get_auth_path_resp);
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}
