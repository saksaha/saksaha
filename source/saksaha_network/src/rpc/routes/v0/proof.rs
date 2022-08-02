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
use std::{fmt::Error, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthPathRequest {
    pub location: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthPathResponse {
    pub result: Vec<Option<([u8; 32], bool)>>,
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
            Ok(n) => match n {
                Some(v) => {
                    let split = loc.split("_");

                    let vec: Vec<&str> = split.collect();

                    let direction = vec[vec.len() - 1];

                    let direction = direction.parse::<u128>();

                    let direction = match direction {
                        Ok(d) => d,
                        Err(err) => {
                            return router::make_error_response(
                                route_state.resp,
                                Some(route_state.id),
                                err.into(),
                            );
                        }
                    };

                    let direction: bool = {
                        if direction % 2 == 1 {
                            false
                        } else {
                            true
                        }
                    };

                    auth_path.push(Some((v, direction)));
                }
                None => {
                    return router::make_error_response(
                        route_state.resp,
                        Some(route_state.id),
                        format!("cannot get merkle node value").into(),
                    );
                }
            },
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
