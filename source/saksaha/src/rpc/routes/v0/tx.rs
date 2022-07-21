use crate::{
    rpc::{router::RPCResponse, RPCError},
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct SendMintTxBody {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    cm: [u8; 32],
    v: [u8; 32],
    k: [u8; 32],
    s: [u8; 32],
}

#[derive(Deserialize, Debug)]
struct SendPourTxBody {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    #[serde(with = "serde_bytes")]
    pi: Vec<u8>,
    sn_1: [u8; 32],
    sn_2: [u8; 32],
    cm_1: [u8; 32],
    cm_2: [u8; 32],
    merkle_rt: [u8; 32],
}

pub(crate) async fn send_mint_tx(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = serde_json::from_slice::<SendMintTxBody>(&b)?;

    let tx_candidate = TxCandidate::Mint(MintTxCandidate::new(
        rb.created_at,
        rb.data,
        rb.author_sig,
        rb.ctr_addr,
        rb.cm,
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
        Ok(bool) => {
            return Ok(RPCResponse::new_success(String::from("1"), "success"));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}

pub(crate) async fn send_pour_tx(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = serde_json::from_slice::<SendPourTxBody>(&b)?;

    let tx_candidate = TxCandidate::Pour(PourTxCandidate::new(
        rb.created_at,
        rb.data,
        rb.author_sig,
        rb.ctr_addr,
        rb.pi,
        rb.sn_1,
        rb.sn_2,
        rb.cm_1,
        rb.cm_2,
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
        Ok(bool) => {
            return Ok(RPCResponse::new_success(String::from("1"), "success"));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}

#[derive(Deserialize, Debug)]
struct GetTransactionBody {
    hash: String,
}

pub(crate) async fn get_transaction(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = serde_json::from_slice::<SendMintTxBody>(&b)?;

    let body: GetTransactionBody = match serde_json::from_slice(&b) {
        Ok(b) => GetTransactionBody { hash: b },
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    };

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx(&body.hash)
        .await
    {
        Ok(t) => {
            return Ok(RPCResponse::new_success(String::from("1"), t));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}
