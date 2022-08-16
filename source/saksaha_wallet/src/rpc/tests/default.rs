use super::utils;
use crate::rpc::routes::v0::{GetBalanceRequest, GetBalanceResponse};
use hyper::{Body, Client, Method, Request, Uri};
use sak_rpc_interface::{JsonRequest, JsonResponse};

#[tokio::test(flavor = "multi_thread")]
async fn test_call_get_block_with_good_params() {
    sak_test_utils::init_test_log();

    let test_context = utils::make_test_context().await;

    let rpc = test_context.rpc;

    let rpc_port = rpc.get_rpc_port();

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!("http://localhost:{}", rpc_port);

        u.parse().expect("URI should be made")
    };

    let body = {
        let get_balance_req = GetBalanceRequest {
            acc_addr: test_context.acc_addr.clone(),
        };

        let params = serde_json::to_vec(&get_balance_req).unwrap();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_balance".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

        println!("[+] request body str (for debugging): {:#?}", str);

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    println!("power: {:?}", b);

    let json_response =
        serde_json::from_slice::<JsonResponse<GetBalanceResponse>>(&b).unwrap();

    let result = json_response.result.unwrap();

    println!("[+] result: {:?}", result);
}