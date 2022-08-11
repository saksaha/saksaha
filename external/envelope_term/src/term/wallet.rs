use hyper::{Body, Client, Method, Request, Uri};
use sak_rpc_interface::{JsonRequest, JsonResponse};

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
