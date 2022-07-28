use super::utils;
use crate::blockchain::GenesisBlock;
use crate::rpc::routes::v0::{QueryCtrRequest, QueryCtrResponse};
use crate::{
    blockchain::ENVELOPE_CTR_ADDR, rpc::routes::v0::SendPourTxRequest,
};
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use sak_types::PourTxCandidate;
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_call_contract() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

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
            arg: HashMap::with_capacity(10),
            ctr_call_type: CtrCallType::Query,
        };

        let call_ctr_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_string(&call_ctr_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "call_contract".to_string(),
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

    println!("query_result (from rpc response) : {:?}", query_result,);

    assert_eq!(expected_validator, query_result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_reqeust_envelope_send_pour_tx() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let tc_dummy = PourTxCandidate::new_dummy_m1_to_p3_p4();
    let expected_tc_hash = tc_dummy.get_tx_hash().clone();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port(),
        );
        u.parse().expect("URI should be made")
    };

    let body = {
        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from("dst_pk"), "her_pk".to_string());
        arg.insert(String::from("serialized_input"), "dummy".to_string());

        let req = CtrRequest {
            req_type: String::from("open_channel"),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        let send_req = SendPourTxRequest::new(
            tc_dummy.created_at,
            serde_json::to_vec(&req).unwrap(),
            tc_dummy.author_sig,
            Some(ctr_addr),
            tc_dummy.pi,
            tc_dummy.sn_1,
            tc_dummy.sn_2,
            tc_dummy.cm_1,
            tc_dummy.cm_2,
            tc_dummy.merkle_rt,
        );

        let params = serde_json::to_vec(&send_req).unwrap();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_pour_tx".to_string(),
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
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    let send_success = json_response.result.unwrap();

    assert_eq!("success", send_success);

    let is_contain = machine
        .blockchain
        .dist_ledger
        .apis
        .tx_pool_contains(&expected_tc_hash)
        .await;

    assert_eq!(true, is_contain);
}
