use super::utils;
use crate::blockchain::GenesisBlock;
use crate::rpc::routes::v0::{QueryCtrRequest, QueryCtrResponse};
use crate::tests::SaksahaTestUtils;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_rpc_interface::{JsonRequest, JsonResponse};

#[tokio::test(flavor = "multi_thread")]
async fn test_call_contract() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test"]);
    SaksahaTestUtils::init_test(vec!["test"]);

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let genesis_block = GenesisBlock::create().unwrap();
    let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

    let expected_validator = String::from(
        "045739d074b8722891c307e8e75c9607e0b55a80778b42ef5f4640d4949dbf3992f60\
        83b729baef9e9545c4e95590616fd382662a09653f2a966ff524989ae8c0f",
    );

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/call_contract",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let ctr_addr = validator_ctr_addr;
        let req = CtrRequest {
            req_type: "get_validator".to_string(),
            args: vec![],
            ctr_call_type: CtrCallType::Query,
        };

        let call_ctr_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_string(&call_ctr_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "query_ctr".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

        println!("request body str (for debugging): {}", str);

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

    let query_ctr_response = json_response.result.unwrap();
    let query_result = query_ctr_response.result;

    println!(
        "query_result (from rpc response) : {:?}",
        std::str::from_utf8(&query_result).unwrap()
    );

    assert_eq!(expected_validator.as_bytes(), query_result);
}
