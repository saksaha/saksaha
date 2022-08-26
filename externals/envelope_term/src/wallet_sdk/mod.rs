use crate::EnvelopeError;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::CtrRequest;
use sak_rpc_interface::{JsonRequest, JsonResponse};
use sak_types::AccountBalance;
use saksaha_wallet::routes::v0::{SendTxRequest, SendTxResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBalanceResponse {
    pub balance: AccountBalance,
}

pub async fn get_balance_from_wallet(
    wallet_endpoint: String,
    acc_addr: &String,
) -> Result<JsonResponse<GetBalanceResponse>, EnvelopeError> {
    let client = Client::new();
    let uri: Uri = { wallet_endpoint.parse().expect("URI should be made") };
    let params = format!(r#"{{"acc_addr": "{}"}}"#, acc_addr)
        .as_bytes()
        .to_vec();

    let body = {
        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_balance".to_string(),
            params: Some(params),
            id: "evl_id".to_string(),
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

    log::info!("body: {:?}", b);

    let json_response =
        serde_json::from_slice::<JsonResponse<GetBalanceResponse>>(&b)?;

    log::info!(":)");

    Ok(json_response)
}

pub async fn send_tx_pour(
    wallet_endpoint: String,
    acc_addr: String,
    ctr_addr: String,
    ctr_request: CtrRequest,
) -> Result<JsonResponse<SendTxResponse>, EnvelopeError> {
    let client = Client::new();
    let uri: Uri = { wallet_endpoint.parse().expect("URI should be made") };

    let body = {
        let send_req = SendTxRequest {
            acc_addr,
            ctr_addr,
            ctr_request,
        };

        let params = serde_json::to_vec(&send_req)?;

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_pour_tx".to_string(),
            params: Some(params),
            id: "evl_id".to_string(),
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

    let json_response =
        serde_json::from_slice::<JsonResponse<SendTxResponse>>(&b)?;

    Ok(json_response)
}

pub async fn update_wallet(
    wallet_endpoint: String,
    acc_addr: &String,
) -> Result<JsonResponse<String>, EnvelopeError> {
    let client = Client::new();

    let uri: Uri = { wallet_endpoint.parse().expect("URI should be made") };
    let params = format!(r#"{{"acc_addr": "{}"}}"#, acc_addr)
        .as_bytes()
        .to_vec();

    let body = {
        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "update_coin_status".to_string(),
            params: Some(params),
            id: "evl_id".to_string(),
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
