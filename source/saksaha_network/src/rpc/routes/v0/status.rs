use crate::{rpc::RPCError, system::SystemHandle};
use hyper::{Body, Request, Response, StatusCode};
use hyper_rpc_router::{make_success_response, Params, RouteState};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodeStatusResponse {
    addr_vec: Vec<String>,
    peer_vec: Vec<String>,
}

pub(in crate::rpc) async fn get_status(
    route_state: RouteState,
    _params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let addr_vec = sys_handle
        .p2p_monitor
        .p2p_discovery
        .addr_table
        .get_status()
        .await;

    let peer_vec = sys_handle.p2p_monitor.peer_table.get_status().await;

    return make_success_response(
        route_state,
        GetNodeStatusResponse { addr_vec, peer_vec },
    );
}
