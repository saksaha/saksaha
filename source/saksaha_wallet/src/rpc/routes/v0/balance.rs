use crate::rpc::ctx::RouteCtx;
use hyper::{Body, Response};
use hyper_rpc_router::{
    require_params_parsed, require_some_params, Params, RouteState,
};
use sak_logger::debug;
use sak_types::AccountBalance;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBalanceRequest {
    pub acc_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBalanceResponse {
    pub balance: AccountBalance,
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

    debug!("params: {:?}", String::from_utf8(params.clone()));

    let rb: GetBalanceRequest = require_params_parsed!(route_state, &params);

    debug!("rb: {:#?}", rb);

    match ctx.wallet.get_balance(&rb.acc_addr).await {
        Ok(b) => {
            let balance = GetBalanceResponse { balance: b };

            hyper_rpc_router::make_success_response(route_state, balance)
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
