use crate::rpc::ctx::RouteCtx;
use hyper::{Body, Response};
use hyper_rpc_router::{require_params_parsed, require_some_params, Params, RouteState};
use sak_logger::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct UpdateCoinStatusRequest {
    pub acc_addr: String,
}

pub(in crate::rpc) async fn update_coin_status(
    route_state: RouteState,
    params: Params,
    ctx: Arc<RouteCtx>,
) -> Response<Body> {
    println!("11");

    debug!("update_coin_status request handling");

    let params = require_some_params!(
        route_state,
        params,
        "update_coin_status should contain params",
    );

    // debug!("\tparams: {:?}", String::from_utf8(params.clone()));

    let rb: UpdateCoinStatusRequest = require_params_parsed!(route_state, &params);

    // debug!("\trb: {:#?}", rb);

    match ctx.wallet.update_coin_status().await {
        Ok(_) => {
            let response_msg = String::from("success");

            hyper_rpc_router::make_success_response(route_state, response_msg)
        }
        Err(err) => {
            return hyper_rpc_router::make_error_response(
                route_state.resp,
                Some(route_state.id),
                format!("some error, err: {:?}", err).into(),
            )
        }
    }
}
