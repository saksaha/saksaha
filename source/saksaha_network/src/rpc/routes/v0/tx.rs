use crate::system::SystemHandle;
use hyper::{Body, Response};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed, require_some_params, Params,
    RouteState,
};
use sak_crypto::encode_hex;
use sak_rpc_interface::{SendMintTxRequest, SendPourTxRequest};
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(in crate::rpc) async fn send_mint_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "send_mint_tx should contain params",);

    let rb: SendMintTxRequest = require_params_parsed!(route_state, &params);

    let tx_candidate = TxCandidate::Mint(MintTxCandidate::new(
        rb.created_at,
        rb.data,
        rb.author_sig,
        rb.ctr_addr,
        rb.cms,
        rb.v,
        rb.k,
        rb.s,
    ));

    match sys_handle
        .machine
        .ledger
        // .dist_ledger
        .send_tx(tx_candidate)
        .await
    {
        Ok(_) => {
            return make_success_response(route_state, "success");
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}

pub(in crate::rpc) async fn send_pour_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "send_pour_tx should contain params",);

    let rb: SendPourTxRequest = require_params_parsed!(route_state, &params);

    let tx_candidate = TxCandidate::Pour(PourTxCandidate::new(
        rb.created_at,
        rb.data,
        rb.author_sig,
        rb.ctr_addr,
        rb.pi,
        rb.sns,
        rb.cms,
        rb.merkle_rts,
    ));

    match sys_handle
        .machine
        .ledger
        // .dist_ledger
        .send_tx(tx_candidate)
        .await
    {
        Ok(tx_hash) => make_success_response(route_state, tx_hash),
        Err(err) => make_error_response(route_state.resp, Some(route_state.id), err),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxRequest {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxResponse {
    pub tx: Option<Tx>,
}

pub(in crate::rpc) async fn get_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(route_state, params, "get_tx should contain params",);

    let rb: GetTxRequest = require_params_parsed!(route_state, &params);

    match sys_handle.machine.ledger.get_tx(&rb.hash).await {
        Ok(tx) => {
            let get_tx_resp = GetTxResponse { tx };

            return make_success_response(route_state, get_tx_resp);
        }
        Err(err) => {
            return make_error_response(route_state.resp, Some(route_state.id), err.into());
        }
    }
}
