use crate::rpc::ctx::RouteCtx;
use hyper::{Body, Response};
use hyper_rpc_router::{require_params_parsed, require_some_params, Params, RouteState};
use sak_contract_std::CtrRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendTxRequest {
    pub acc_addr: String,
    pub ctr_addr: String,
    pub ctr_request: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendTxResponse {
    pub result: String,
}

pub(in crate::rpc) async fn send_pour_tx(
    route_state: RouteState,
    params: Params,
    ctx: Arc<RouteCtx>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "send_tx should contain params",);

    let rb: SendTxRequest = require_params_parsed!(route_state, &params);

    let wallet = &ctx.wallet;

    let res = wallet
        .send_pour_tx(rb.acc_addr, rb.ctr_addr, rb.ctr_request)
        .await;

    match res {
        Ok(r) => {
            let response = SendTxResponse { result: r };

            hyper_rpc_router::make_success_response(route_state, response)
        }
        Err(err) => {
            println!("err: {}", err);

            hyper_rpc_router::make_error_response(route_state.resp, Some(route_state.id), err)
        }
    }
}
