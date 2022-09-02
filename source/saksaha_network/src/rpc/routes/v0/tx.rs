use crate::system::SystemHandle;
use hyper::{Body, Response};
use hyper_rpc_router::{
    make_error_response, make_success_response, require_params_parsed,
    require_some_params, Params, RouteState,
};
use sak_rpc_interface::{SendMintTxRequest, SendPourTxRequest};
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use type_extension::U8Arr32;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SendMintTxRequest {
//     created_at: String,
//     #[serde(with = "serde_bytes")]
//     data: Vec<u8>,
//     author_sig: String,
//     ctr_addr: Option<String>,
//     // cm: [u8; 32],
//     // #[serde(with = "serde_bytes")]
//     cms: Vec<[u8; 32]>,
//     // cm_count: u128,
//     v: [u8; 32],
//     k: [u8; 32],
//     s: [u8; 32],
// }

// impl SendMintTxRequest {
//     pub fn new(
//         created_at: String,
//         data: Vec<u8>,
//         author_sig: String,
//         ctr_addr: Option<String>,
//         // cm: [u8; 32],
//         cms: Vec<[u8; 32]>,
//         // cm_count: u128,
//         v: [u8; 32],
//         k: [u8; 32],
//         s: [u8; 32],
//     ) -> SendMintTxRequest {
//         SendMintTxRequest {
//             created_at,
//             data,
//             author_sig,
//             ctr_addr,
//             cms,
//             // cm_count,
//             v,
//             k,
//             s,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub(in crate::rpc) struct SendPourTxRequest {
//     created_at: String,
//     #[serde(with = "serde_bytes")]
//     data: Vec<u8>,
//     author_sig: String,
//     ctr_addr: Option<String>,
//     #[serde(with = "serde_bytes")]
//     pi: Vec<u8>,
//     sn_1: [u8; 32],
//     // sn_2: [u8; 32],
//     cms: Vec<[u8; 32]>,
//     // cm_count: u128,
//     // cm_1: [u8; 32],
//     // cm_2: [u8; 32],
//     merkle_rt: [u8; 32],
// }

// impl SendPourTxRequest {
//     pub fn new(
//         created_at: String,
//         data: Vec<u8>,
//         author_sig: String,
//         ctr_addr: Option<String>,
//         pi: Vec<u8>,
//         sn_1: U8Arr32,
//         // sn_2: [u8; 32],
//         cms: Vec<[u8; 32]>,
//         // cm_count: u128,
//         // cm_1: [u8; 32],
//         // cm_2: [u8; 32],
//         merkle_rt: [u8; 32],
//     ) -> SendPourTxRequest {
//         SendPourTxRequest {
//             created_at,
//             data,
//             author_sig,
//             ctr_addr,
//             pi,
//             sn_1,
//             cms,
//             // cm_count,
//             // sn_2,
//             // cm_1,
//             // cm_2,
//             merkle_rt,
//         }
//     }
// }

pub(in crate::rpc) async fn send_mint_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(
        route_state,
        params,
        "send_mint_tx should contain params",
    );

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
        .blockchain
        .dist_ledger
        .apis
        .send_tx(tx_candidate)
        .await
    {
        Ok(_) => {
            return make_success_response(route_state, "success");
        }
        Err(err) => {
            return make_error_response(
                route_state.resp,
                Some(route_state.id),
                err.into(),
            );
        }
    }
}

pub(in crate::rpc) async fn send_pour_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(
        route_state,
        params,
        "send_pour_tx should contain params",
    );

    let rb: SendPourTxRequest = require_params_parsed!(route_state, &params);

    let tx_candidate = TxCandidate::Pour(PourTxCandidate::new(
        rb.created_at,
        rb.data,
        rb.author_sig,
        rb.ctr_addr,
        rb.pi,
        // rb.sn_1,
        rb.sns,
        rb.cms,
        // rb.cm_count,
        // rb.sn_2,
        // rb.cm_1,
        // rb.cm_2,
        rb.merkle_rt,
    ));

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .send_tx(tx_candidate)
        .await
    {
        Ok(tx_hash) => {
            return make_success_response(route_state, tx_hash);
        }
        Err(err) => {
            return make_error_response(
                route_state.resp,
                Some(route_state.id),
                err.into(),
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxRequest {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxResponse {
    //
    pub tx: Option<Tx>,
}

pub(in crate::rpc) async fn get_tx(
    route_state: RouteState,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Response<Body> {
    let params = require_some_params!(
        route_state,
        params,
        "get_tx should contain params",
    );

    let rb: GetTxRequest = require_params_parsed!(route_state, &params);

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx(&rb.hash)
        .await
    {
        Ok(tx) => {
            let get_tx_resp = GetTxResponse { tx };

            return make_success_response(route_state, get_tx_resp);
        }
        Err(err) => {
            return make_error_response(
                route_state.resp,
                Some(route_state.id),
                err.into(),
            );
        }
    }
}
