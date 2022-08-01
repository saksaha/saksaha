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
pub(crate) struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryCtrResponse {
    pub result: String,
}

pub(in crate::rpc) async fn query_ctr(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = router::require_some_params!(
        route_state,
        params,
        "query_ctr should contain params",
    );

    let rb: QueryCtrRequest =
        router::require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .query_ctr(&rb.ctr_addr, rb.req)
        .await
    {
        Ok(t) => {
            return router::make_success_response(
                route_state,
                QueryCtrResponse { result: t },
            );
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
