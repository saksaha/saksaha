use crate::{
    rpc::{
        router::{self, Params},
        RPCError,
    },
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
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

pub(crate) async fn get_status(
    res: Response<Body>,
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let addr_vec = sys_handle
        .p2p_monitor
        .p2p_discovery
        .addr_table
        .get_status()
        .await;

    let peer_vec = sys_handle.p2p_monitor.peer_table.get_status().await;

    return Ok(router::make_success_response(
        res,
        id,
        GetNodeStatusResponse { addr_vec, peer_vec },
    ));
}
