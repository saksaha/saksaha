use crate::SaksahaSDKError;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use sak_types::U8Array;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const A: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrResponse {
    pub result: String,
}

pub fn new_empty_32_temp() -> [u8; 32] {
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendPourTxRequest {
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

pub async fn send_tx_pour(
    ctr_addr: String,
    req_type: String,
    arg: HashMap<String, String>,
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        // ***** Need to change dummy values to real values
        let send_req = SendPourTxRequest::new(
            String::from("created_at_1"),
            serde_json::to_vec(&req).unwrap(),
            String::from("author_sig_1"),
            Some(ctr_addr),
            vec![11, 11, 11],
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
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

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let json_response =
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    Ok(json_response)
}

pub async fn send_tx_mint(
    ctr_addr: String,
    req_type: String,
    arg: HashMap<String, String>,
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        // ***** Need to change dummy values to real values
        let send_req = SendMintTxRequest::new(
            String::from("created_at_1"),
            serde_json::to_vec(&req).unwrap(),
            String::from("author_sig_1"),
            Some(ctr_addr),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
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

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let json_response =
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    Ok(json_response)
}

pub async fn call_contract(
    ctr_addr: String,
    req_type: String,
    arg: HashMap<String, String>,
) -> Result<JsonResponse<QueryCtrResponse>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            arg,
            ctr_call_type: CtrCallType::Query,
        };

        let send_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "call_contract".to_string(),
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

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let json_response =
        serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b).unwrap();

    Ok(json_response)
}
