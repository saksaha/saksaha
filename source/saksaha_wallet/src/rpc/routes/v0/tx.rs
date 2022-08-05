use crate::rpc::ctx::RouteCtx;
use hyper::{Body, Response};
use hyper_rpc_router::{
    require_params_parsed, require_some_params, Params, RouteState,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type RequestArgs = Vec<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct SendTxRequest {
    // created_at: String,
    // #[serde(with = "serde_bytes")]
    // data: Vec<u8>,
    // author_sig: String,
    // ctr_addr: Option<String>,
    val: usize,
    ctr_addr: String,
    req_type: String,
    args: RequestArgs,
}

pub(in crate::rpc) async fn send_tx(
    route_state: RouteState,
    params: Params,
    ctx: Arc<RouteCtx>,
) -> Response<Body> {
    let params = require_some_params!(
        route_state,
        params,
        "send_tx should contain params",
    );

    let rb: SendTxRequest = require_params_parsed!(route_state, &params);

    let _ = ctx.wallet.apis.send_tx(rb.val).await;

    let is_success = true;

    if is_success {
        hyper_rpc_router::make_success_response(route_state, "send tx success")
    } else {
        return hyper_rpc_router::make_error_response(
            route_state.resp,
            Some(route_state.id),
            "some error".into(),
        );
    }

    // let tx_candidate = TxCandidate::Pour(PourTxCandidate::new(
    //     rb.created_at,
    //     rb.data,
    //     rb.author_sig,
    //     rb.ctr_addr,
    //     rb.pi,
    //     rb.sn_1,
    //     rb.sn_2,
    //     rb.cm_1,
    //     rb.cm_2,
    //     rb.merkle_rt,
    // ));

    // match sys_handle
    //     .machine
    //     .blockchain
    //     .dist_ledger
    //     .apis
    //     .send_tx(tx_candidate)
    //     .await
    // {
    //     Ok(bool) => {
    //         return make_success_response(route_state, "success");
    //     }
    //     Err(err) => {
    //         return make_error_response(
    //             route_state.resp,
    //             Some(route_state.id),
    //             err.into(),
    //         );
    //     }
    // }
}
