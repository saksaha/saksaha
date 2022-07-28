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
pub struct SendMintTxRequest {
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

impl SendMintTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cm: [u8; 32],
        v: [u8; 32],
        k: [u8; 32],
        s: [u8; 32],
    ) -> SendMintTxRequest {
        SendMintTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            cm,
            v,
            k,
            s,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct SendPourTxRequest {
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

impl SendPourTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        pi: Vec<u8>,
        sn_1: [u8; 32],
        sn_2: [u8; 32],
        cm_1: [u8; 32],
        cm_2: [u8; 32],
        merkle_rt: [u8; 32],
    ) -> SendPourTxRequest {
        SendPourTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            pi,
            sn_1,
            sn_2,
            cm_1,
            cm_2,
            merkle_rt,
        }
    }
}

pub(crate) async fn send_mint_tx(
    res: Response<Body>,
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb = router::parse_params::<SendMintTxRequest>(&params)?;

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
            return Ok(router::make_success_response(res, id, "success"));
        }
        Err(err) => {
            return Ok(router::make_error_response(
                res,
                Some(String::from("1")),
                err.into(),
            ));
        }
    }
}

pub(crate) async fn send_pour_tx(
    res: Response<Body>,
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb = router::parse_params::<SendPourTxRequest>(&params)?;

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
            return Ok(router::make_success_response(res, id, "success"));
        }
        Err(err) => {
            return Ok(router::make_error_response(
                res,
                Some(String::from("1")),
                err.into(),
            ));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxRequest {
    pub hash: String,
}

pub(crate) async fn get_tx(
    res: Response<Body>,
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb = router::parse_params::<GetTxRequest>(&params)?;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx(&rb.hash)
        .await
    {
        Ok(t) => {
            return Ok(router::make_success_response(res, id, t));
        }
        Err(err) => {
            return Ok(router::make_error_response(
                res,
                Some(String::from("1")),
                err.into(),
            ));
        }
    }
}
