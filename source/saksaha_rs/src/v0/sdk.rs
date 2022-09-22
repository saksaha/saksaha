use crate::SaksahaSDKError;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, CtrRequest, RequestArgs};
use sak_crypto::encode_hex;
use sak_dist_ledger_meta::CM_TREE_DEPTH;
use sak_rpc_interface::{JsonRequest, JsonResponse, SendMintTxRequest, SendPourTxRequest};
use sak_types::{Cm, CmIdx, Tx};
use serde::{Deserialize, Serialize};
use std::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrResponse {
    pub result: Vec<u8>,
    // pub result: String,
}

pub fn new_empty_32_temp() -> [u8; 32] {
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
}

pub async fn send_tx_pour(
    saksaha_endpoint: String,
    sns: Vec<[u8; 32]>,
    cms: Vec<[u8; 32]>,
    merkle_rts: Vec<[u8; 32]>,
    pi: Vec<u8>,
    ctr_addr: String,
    ctr_request: CtrRequest,
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    let client = Client::new();
    let uri: Uri = { saksaha_endpoint.parse().expect("URI should be made") };

    let body = {
        let ctr_request = serde_json::to_vec(&ctr_request)?;
        let sig = String::from("author_sig_1");
        let created_at = String::from(format!("created_at_{:?}", time::SystemTime::now()));

        // *** Need to change dummy values to real values
        let send_req = SendPourTxRequest::new(
            created_at,
            ctr_request,
            sig,
            Some(ctr_addr),
            pi,
            sns,
            cms,
            merkle_rts,
        );

        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_pour_tx".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}

pub async fn send_tx_mint(
    rpc_port: u16,

    ctr_addr: Option<String>,
    req_type: String,
    args: RequestArgs,

    cms: Vec<[u8; 32]>,
    v: [u8; 32],
    k: [u8; 32],
    s: [u8; 32],
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    // let endpoint_test = "http://localhost:34418/rpc/v0";
    let endpoint = format!(
        "{}{}{}",
        "http://localhost:",
        rpc_port.to_string(),
        "/rpc/v0"
    );

    let client = Client::new();
    let uri: Uri = { endpoint.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        let send_req = SendMintTxRequest::new(
            String::from("created_at_1"),
            serde_json::to_vec(&req)?,
            String::from("author_sig_1"),
            ctr_addr,
            cms,
            v,
            k,
            s,
        );

        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_mint_tx".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}

pub async fn query_ctr(
    saksaha_endpoint: String,
    ctr_addr: String,
    req_type: String,
    args: RequestArgs,
) -> Result<JsonResponse<QueryCtrResponse>, SaksahaSDKError> {
    let client = Client::new();

    let uri: Uri = { saksaha_endpoint.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            args,
            ctr_call_type: CtrCallType::Query,
        };

        let send_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_vec(&send_req)?;

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "query_ctr".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b)?;

    // println!("json_response: {:?}", json_response);
    Ok(json_response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCmIdxRequest {
    pub cm: Cm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCmIdxResponse {
    pub cm_idx: Option<CmIdx>,
}

pub async fn get_cm_idx(
    saksaha_endpoint: String,
    cm: [u8; 32],
) -> Result<JsonResponse<GetCmIdxResponse>, SaksahaSDKError> {
    let client = Client::new();
    let uri: Uri = { saksaha_endpoint.parse().expect("URI should be made") };

    let body = {
        let req = GetCmIdxRequest { cm };

        let params = serde_json::to_string(&req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_cm_idx".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<GetCmIdxResponse>>(&b)?;

    Ok(json_response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxRequest {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxResponse {
    pub tx: Option<Tx>,
}

pub async fn get_tx(
    saksaha_endpoint: String,
    hash: String,
) -> Result<JsonResponse<GetTxResponse>, SaksahaSDKError> {
    let client = Client::new();
    let uri: Uri = { saksaha_endpoint.parse().expect("URI should be made") };

    let body = {
        let req = GetTxRequest { hash };

        let params = serde_json::to_string(&req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_tx".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<GetTxResponse>>(&b)?;

    Ok(json_response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthPathRequest {
    pub cm_idx: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthPathResponse {
    pub auth_path: Vec<([u8; 32], bool)>,
}

pub async fn get_auth_path(
    saksaha_endpoint: String,
    idx: u128,
) -> Result<JsonResponse<GetAuthPathResponse>, SaksahaSDKError> {
    let client = Client::new();
    let uri: Uri = { saksaha_endpoint.parse().expect("URI should be made") };

    let body = {
        let send_req = GetAuthPathRequest { cm_idx: idx };
        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_auth_path".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<GetAuthPathResponse>>(&b)?;

    Ok(json_response)
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GenProofRequest {
//     // coin_1_old
//     pub addr_pk_1_old: [u8; 32],
//     pub addr_sk_1_old: [u8; 32],
//     pub rho_1_old: [u8; 32],
//     pub r_1_old: [u8; 32],
//     pub s_1_old: [u8; 32],
//     pub v_1_old: [u8; 32],
//     pub cm_1_old: [u8; 32],
//     pub auth_path_1_old: [Option<([u8; 32], bool)>; CM_TREE_DEPTH as usize],

//     // coin_1_new
//     pub addr_pk_1_new: [u8; 32],
//     pub rho_1_new: [u8; 32],
//     pub r_1_new: [u8; 32],
//     pub s_1_new: [u8; 32],
//     pub v_1_new: [u8; 32],

//     // coin_2_new
//     pub addr_pk_2_new: [u8; 32],
//     pub rho_2_new: [u8; 32],
//     pub r_2_new: [u8; 32],
//     pub s_2_new: [u8; 32],
//     pub v_2_new: [u8; 32],
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GenProofResponse {
//     pub pi: Vec<Option<[u8; 32]>>,
// }
