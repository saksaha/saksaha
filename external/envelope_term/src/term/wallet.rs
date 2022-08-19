use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::CtrRequest;
use sak_rpc_interface::{JsonRequest, JsonResponse};
use saksaha_wallet::routes::v0::SendTxRequest;

use crate::EnvelopeError;

pub async fn get_balance_from_wallet(
    user_id: &String,
) -> Result<JsonResponse<String>, EnvelopeError> {
    let endpoint = "http://localhost:36612/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint.parse().expect("URI should be made") };
    let params = format!(r#"{{"id": "{}","key": "user_1_key"}}"#, user_id)
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

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}

pub async fn send_tx_pour(
    acc_addr: String,
    ctr_addr: String,
    ctr_request: CtrRequest,
) -> Result<JsonResponse<String>, EnvelopeError> {
    let endpoint_test = "http://localhost:36612/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

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

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}
