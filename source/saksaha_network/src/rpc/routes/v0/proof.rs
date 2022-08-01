use crate::{
    rpc::{
        router::{self, Params, RouteState},
        RPCError,
    },
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthPathRequest {
    pub location: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthPathResponse {
    pub result: Vec<Option<[u8; 32]>>,
}

pub(in crate::rpc) async fn get_auth_path(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = router::require_some_params!(
        route_state,
        params,
        "get_auth_path should contain params",
    );

    let rb: AuthPathRequest =
        router::require_params_parsed!(route_state, &params);

    let locations = rb.location;

    let mut auth_path = Vec::new();

    for loc in locations {
        match sys_handle
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_merkle_node(&loc)
            .await
        {
            Ok(n) => {
                auth_path.push(n);
            }
            Err(err) => {
                return router::make_error_response(
                    route_state.resp,
                    Some(route_state.id),
                    err.into(),
                );
            }
        }
    }

    router::make_success_response(
        route_state,
        AuthPathResponse { result: auth_path },
    )
}
