use crate::{
    rpc::{RPCError, RPCResponse},
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
struct NodeStatus {
    addr_vec: Vec<String>,
    peer_vec: Vec<String>,
}

pub(crate) async fn get_status(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let addr_vec = sys_handle
        .p2p_monitor
        .p2p_discovery
        .addr_table
        .get_status()
        .await;

    let peer_vec = sys_handle.p2p_monitor.peer_table.get_status().await;

    let result = NodeStatus {
        addr_vec, //
        peer_vec,
    };

    return Ok(RPCResponse::new_success(String::from("1"), result));
}

