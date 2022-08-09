use crate::rpc::ctx::RouteCtx;
use hyper::{Body, Response};
use hyper_rpc_router::{
    require_params_parsed, require_some_params, Params, RouteState,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBalanceRequest {
    pub id: String,
    pub key: String,
}

pub(in crate::rpc) async fn get_balance(
    route_state: RouteState,
    params: Params,
    ctx: Arc<RouteCtx>,
) -> Response<Body> {
    debug!("get_balance request handling");

    let params = require_some_params!(
        route_state,
        params,
        "get_balance should contain params",
    );

    let rb: GetBalanceRequest = require_params_parsed!(route_state, &params);

    let _ = ctx.wallet.apis.get_balance(rb.id, rb.key).await;

    let is_success = true;

    if is_success {
        hyper_rpc_router::make_success_response(
            route_state,
            "get balance success",
        )
    } else {
        return hyper_rpc_router::make_error_response(
            route_state.resp,
            Some(route_state.id),
            "some error".into(),
        );
    }
}
