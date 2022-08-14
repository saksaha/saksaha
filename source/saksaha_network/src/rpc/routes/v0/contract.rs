use crate::{rpc::RPCError, system::SystemHandle};
use hyper::{Body, Request, Response, StatusCode};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed,
    require_some_params, Params, RouteState,
};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenChBody {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}

pub(in crate::rpc) async fn query_ctr(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(
        route_state,
        params,
        "query_ctr should contain params",
    );

    let rb: QueryCtrRequest = require_params_parsed!(route_state, &params);
    println!("in query_ctr, {:?}", rb);

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .query_ctr(&rb.ctr_addr, rb.req)
        .await
    {
        Ok(t) => {
            println!("t: {:?}", t);

            let result: OpenChBody = serde_json::from_slice(&t).unwrap();
            // println!("{:?}", result);
            // let result: OpenChBody = require_params_parsed!(route_state, &t);
            // let result: String = require_params_parsed!(route_state, &t);
            // let result = std::str::from_utf8(&t).unwrap_or("");
            // println!("{:?}", result);
            // let result: String =
            //     serde_json::from_str(result).unwrap_or(String::default());
            println!("1111 {:?}", result);

            return make_success_response(
                route_state,
                QueryCtrResponse {
                    result: serde_json::to_string(&result).unwrap(),
                },
            );
        }
        Err(err) => {
            println!("fail");

            return make_error_response(
                route_state.resp,
                Some(route_state.id),
                err.into(),
            );
        }
    }
}
