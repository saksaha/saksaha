use crate::system::SystemHandle;
use async_trait::async_trait;
use hyper::{Body, Response};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed, require_some_params, Params,
    RouteState,
};
use sak_contract_std::{CtrRequest, CtrRequestData};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryCtrResponse {
    pub result: Vec<u8>,
}

pub(in crate::rpc) async fn query_ctr(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "query_ctr should contain params",);

    let rb: QueryCtrRequest = require_params_parsed!(route_state, &params);

    let ctr_request = CtrRequest {
        ctr_addr: rb.ctr_addr.clone(),
        req_type: rb.req.req_type,
        args: rb.req.args,
        ctr_call_type: rb.req.ctr_call_type,
    };

    let res = sys_handle.machine.ledger.execute_ctr(ctr_request).await;

    match res {
        Ok(result) => make_success_response(route_state, QueryCtrResponse { result }),
        Err(err) => make_error_response(route_state.resp, Some(route_state.id), err),
    }
}
