use super::utils;
use crate::rpc::router::{JsonRequest, JsonResponse};
use crate::rpc::routes::v0::{GetBlockRequest, GetBlockResponse};
use hyper::{Body, Client, Method, Request, Uri};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_get_block() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let block_candidate_same = utils::make_dummy_tx_pour_block();

    let original_block_hash = {
        let block_hash = match machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(Some(block_candidate_same))
            .await
        {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        block_hash.unwrap()
    };

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_block",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let r = GetBlockRequest {
            block_hash: original_block_hash.to_string(),
        };

        let params = serde_json::to_string(&r).unwrap().as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_block".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

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
        serde_json::from_slice::<JsonResponse<GetBlockResponse>>(&b).unwrap();

    let result = json_response.result.unwrap();

    let block_acquired = result.block.unwrap();

    println!(
        "block hash (from rpc response) : {:?}",
        block_acquired.get_block_hash(),
    );

    assert_eq!(block_acquired.get_block_hash(), &original_block_hash);
}
