use super::utils::{self, TestContext};
use crate::ledger::GenesisBlock;
use crate::rpc::routes::v0::{QueryCtrRequest, QueryCtrResponse};
use crate::tests::SaksahaTestUtils;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, CtrRequest, CtrRequestData};
use sak_credential::CredentialProfile;
use sak_mrs_contract::ReserveSlotParams;
use sak_rpc_interface::{JsonRequest, JsonResponse, SendPourTxRequest};
use sak_types::{mock_pour_tc_random, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_call_contract() {
    let test_credential_1 = CredentialProfile::test_1();

    SaksahaTestUtils::init_test(&[&test_credential_1.public_key_str]);

    let TestContext {
        rpc,
        rpc_socket_addr,
        ..
    } = utils::make_test_context(test_credential_1.secret, test_credential_1.public_key_str).await;

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
        let req = CtrRequestData {
            req_type: "get_validator".to_string(),
            args: vec![],
            ctr_call_type: CtrCallType::Execute,
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

    let json_response = serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b).unwrap();

    let query_ctr_response = json_response.result.unwrap();
    let query_result = query_ctr_response.result;

    println!(
        "query_result (from rpc response) : {:?}",
        std::str::from_utf8(&query_result).unwrap()
    );

    assert_eq!(expected_validator.as_bytes(), query_result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_call_mrs_contract() {
    let test_credential_1 = CredentialProfile::test_1();

    SaksahaTestUtils::init_test(&[&test_credential_1.public_key_str]);

    let TestContext {
        rpc,
        rpc_socket_addr,
        ..
    } = utils::make_test_context(test_credential_1.secret, test_credential_1.public_key_str).await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let genesis_block = GenesisBlock::create().unwrap();
    let mrs_ctr_addr = genesis_block.get_mrs_ctr_addr();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/call_contract",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let ctr_addr = mrs_ctr_addr;
        let req = CtrRequestData {
            req_type: "get_slot".to_string(),
            args: vec![],
            ctr_call_type: CtrCallType::Execute,
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

    let json_response = serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b).unwrap();

    // println!("json_reponse: {:?}", json_response);
    let query_ctr_response = json_response.result.unwrap();
    let query_result = query_ctr_response.result;

    println!(
        "query_result (from rpc response) : {:?}",
        std::str::from_utf8(&query_result).unwrap()
    );

    // assert_eq!(expected_validator.as_bytes(), query_result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reserve_mrs_slot_and_get_it_back() {
    let test_credential_1 = CredentialProfile::test_1();

    SaksahaTestUtils::init_test(&[&test_credential_1.public_key_str]);

    let TestContext {
        rpc,
        rpc_socket_addr,
        machine,
    } = utils::make_test_context(test_credential_1.secret, test_credential_1.public_key_str).await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let data: Vec<u8> = {
        let tmp = ReserveSlotParams {
            public_key: test_credential_1.public_key_str,
        };

        serde_json::to_vec(&tmp).unwrap()
    };

    let tc_dummy = match sak_types::mock_pour_tc_random() {
        TxCandidate::Pour(p) => p,
        _ => panic!("pour tx should be exist"),
    };

    let body = {
        let send_req = SendPourTxRequest::new(
            tc_dummy.created_at,
            data,
            tc_dummy.author_sig,
            Some(tc_dummy.ctr_addr),
            tc_dummy.pi,
            tc_dummy.sns,
            tc_dummy.cms,
            tc_dummy.merkle_rts,
        );

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

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

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    let send_success = json_response.result.unwrap();

    assert_eq!("success", send_success);
}
