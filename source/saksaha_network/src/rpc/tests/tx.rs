use super::utils;
use crate::{
    rpc::routes::v0::{GetTxRequest, GetTxResponse},
    tests::TestUtil,
};
use hyper::{Body, Client, Method, Request, Uri};
use sak_rpc_interface::{
    JsonRequest, JsonResponse, SendMintTxRequest, SendPourTxRequest,
};
use sak_types::{BlockCandidate, Tx, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_request_correct_get_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let expected_tx_hash = {
        let blockchain = utils::make_blockchain().await;

        // let dummy_tx = sak_types::mock_pour_tc_m1_to_p3_p4();
        let dummy_tx = sak_types::mock_pour_tc_1();

        let old_tx_hash = (&dummy_tx).get_tx_hash();

        let dist_ledger = blockchain.dist_ledger;

        dist_ledger
            .apis
            .delete_tx(&old_tx_hash)
            .expect("Tx should be deleted");

        let bc = Some(BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![dummy_tx.clone()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: format!("{}", 0),
        });

        dist_ledger.apis.write_block(bc).await.unwrap();

        let tx = dist_ledger
            .apis
            .get_tx(&old_tx_hash.clone())
            .await
            .expect("Tx should be exist")
            .unwrap();

        let tx_hash = tx.get_tx_hash().clone();

        assert_eq!(tx_hash, *old_tx_hash);
        old_tx_hash.clone()
    };

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_tx",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = GetTxRequest {
            hash: expected_tx_hash.clone(),
        };

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_tx".to_string(),
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
        serde_json::from_slice::<JsonResponse<GetTxResponse>>(&b).unwrap();

    let tx_from_res = json_response.result.unwrap();
    let tx_from_res = tx_from_res.tx.unwrap();
    let tx_hash_from_res = tx_from_res.get_tx_hash();

    assert_eq!(&expected_tx_hash, tx_hash_from_res);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_request_wrong_get_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let _expected_tx_hash = {
        let blockchain = utils::make_blockchain().await;

        // let dummy_tx = sak_types::mock_pour_tc_m1_to_p3_p4();
        let dummy_tx = sak_types::mock_pour_tc_1();

        let old_tx_hash = (&dummy_tx).get_tx_hash();

        let dist_ledger = blockchain.dist_ledger;

        dist_ledger
            .apis
            .delete_tx(&old_tx_hash)
            .expect("Tx should be deleted");

        let bc = Some(BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![dummy_tx.clone()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: format!("{}", 0),
        });

        dist_ledger.apis.write_block(bc).await.unwrap();

        let tx = dist_ledger
            .apis
            .get_tx(&old_tx_hash.clone())
            .await
            .expect("Tx should be exist")
            .unwrap();

        let tx_hash = tx.get_tx_hash().clone();

        assert_eq!(tx_hash, *old_tx_hash);
        old_tx_hash.clone()
    };

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_tx",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let false_tx_hash = String::from("false_tx_hash");

    let body = {
        let send_req = GetTxRequest {
            hash: false_tx_hash,
        };

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_tx".to_string(),
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
        serde_json::from_slice::<JsonResponse<GetTxResponse>>(&b).unwrap();

    assert!(json_response.result.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_reqeust_correct_send_pour_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let tc_dummy = if let TxCandidate::Pour(c) = sak_types::mock_pour_tc_1() {
        c
    } else {
        panic!("mock tx candidate should be pour tx candidate");
    };

    let expected_tc_hash = tc_dummy.get_tx_hash().clone();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = SendPourTxRequest::new(
            tc_dummy.created_at,
            tc_dummy.data,
            tc_dummy.author_sig,
            Some(tc_dummy.ctr_addr),
            tc_dummy.pi,
            tc_dummy.sns,
            tc_dummy.cms,
            tc_dummy.merkle_rt,
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

    let json_response =
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    let result_hash = json_response.result.unwrap();

    assert_eq!(expected_tc_hash, result_hash);

    let is_contain = machine
        .blockchain
        .dist_ledger
        .apis
        .tx_pool_contains(&expected_tc_hash)
        .await;

    assert!(is_contain);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_reqeust_wrong_send_pour_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = String::from("False request");

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

    let json_response =
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    assert!(json_response.result == None);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_reqeust_correct_send_mint_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    // let tc_dummy = MintTxCandidate::new_dummy_2();
    let tc_dummy = sak_types::mock_mint_tc_1()
        .into_mint_tx_candidate()
        .unwrap();

    let expected_tc_hash = tc_dummy.get_tx_hash().clone();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_mint_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = SendMintTxRequest::new(
            tc_dummy.created_at,
            tc_dummy.data,
            tc_dummy.author_sig,
            Some(tc_dummy.ctr_addr),
            tc_dummy.cms,
            tc_dummy.v,
            tc_dummy.k,
            tc_dummy.s,
        );

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_mint_tx".to_string(),
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

    assert!(is_contain);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_reqeust_wrong_send_mint_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_mint_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = String::from("False request");

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_mint_tx".to_string(),
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

    assert!(json_response.result == None);
}
