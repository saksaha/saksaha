use super::utils;
use crate::rpc::response::JsonResponse;
use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Uri};
use sak_types::{BlockCandidate, PourTxCandidate, TxCandidate};
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_send_wrong_transaction() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, _) = utils::make_test_context().await;

    let _rpc_server = tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(Body::from("123"))
        .expect("request builder should be made");

    let _result = match client.request(req).await {
        Ok(mut res) => {
            let body = hyper::body::aggregate(&mut res)
                .await
                .expect("body should be parsed");

            // let _: ErrorResponse = match serde_json::from_reader(body.reader())
            // {
            //     Ok(e) => e,
            //     Err(err) => {
            //         panic!("Response should be 'error_response', {}", err);
            //     }
            // };
        }
        Err(err) => {
            panic!("error: {}", err);
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_request_wrong_transaction_hash() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    {
        let blockchain = utils::make_blockchain().await;

        let dummy_tx = TxCandidate::new_dummy_pour_m1_to_p3_p4();

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

        let _tx_hash = dist_ledger
            .apis
            .send_tx(dummy_tx)
            .await
            .expect("Tx should be written");
    }

    let (rpc, rpc_socket_addr, _) = utils::make_test_context().await;

    let _rpc_server = tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_transaction",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(Body::from(r#"{"hash": "1q2w3e4r"}"#))
        .expect("request builder should be made");

    let _res = match client.request(req).await {
        Ok(res) => {
            let body = hyper::body::aggregate(res)
                .await
                .expect("body should be parsed");

            // let _: ErrorResponse = match serde_json::from_reader(body.reader())
            // {
            //     Ok(e) => {
            //         log::info!("{:?}", e);
            //         e
            //     }
            //     Err(err) => {
            //         panic!("Response should be 'error_response', {}", err);
            //     }
            // };
        }
        Err(err) => {
            panic!("error: {}", err);
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_request_correct_transaction_hash() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let tx_hash = {
        let blockchain = utils::make_blockchain().await;

        let dummy_tx = TxCandidate::new_dummy_pour_m1_to_p3_p4();

        let old_tx_hash = (&dummy_tx).get_tx_hash();

        let dist_ledger = blockchain.dist_ledger;

        dist_ledger
            .apis
            .delete_tx(&old_tx_hash)
            .expect("Tx should be deleted");

        // dist_ledger
        //     .apis
        //     .send_tx(dummy_tx.clone())
        //     .await
        //     .expect("Tx should be written");

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

    let (rpc, rpc_socket_addr, _) = utils::make_test_context().await;

    let _rpc_server = tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_transaction",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(Body::from(format!(
            r#"
                    {{"hash": "{}"}}
                "#,
            tx_hash,
        )))
        .expect("request builder should be made");

    let _res = match client.request(req).await {
        Ok(res) => {
            let body = hyper::body::aggregate(res)
                .await
                .expect("body should be parsed");

            // let _: JsonResponse = match serde_json::from_reader(body.reader()) {
            //     Ok(e) => {
            //         log::info!("{:?}", e);
            //         e
            //     }
            //     Err(err) => {
            //         panic!("Response should be 'error_response', {}", err);
            //     }
            // };
        }
        Err(err) => {
            panic!("error: {}", err);
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
async fn test_if_send_transaction_puts_tx_into_tx_pool() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let tc_dummy = PourTxCandidate::new_dummy_m1_to_p3_p4();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;
    let _rpc_server = tokio::spawn(async move { rpc.run().await });
    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .body(Body::from(format!(
            r#"
                    {{
                        "pi": {:?},
                        "author_sig": "{}",
                        "created_at": "{}",
                        "data": {:?},
                        "ctr_addr": {:?},
                        "sn_1": {:?},
                        "sn_2": {:?},
                        "cm_1": {:?},
                        "cm_2": {:?},
                        "merkle_rt": {:?}                        
                    }}
                "#,
            tc_dummy.pi,
            tc_dummy.author_sig,
            tc_dummy.created_at,
            tc_dummy.data,
            tc_dummy.ctr_addr,
            tc_dummy.sn_1,
            tc_dummy.sn_2,
            tc_dummy.cm_1,
            tc_dummy.cm_2,
            tc_dummy.merkle_rt,
        )))
        .expect("request builder should be made");

    let _res = match client.request(req).await {
        Ok(res) => {
            let body = hyper::body::aggregate(res)
                .await
                .expect("body should be parsed");

            // let _: JsonResponse = match serde_json::from_reader(body.reader()) {
            //     Ok(e) => {
            //         log::info!("log info dbg {:?}", e);
            //         e
            //     }
            //     Err(err) => {
            //         panic!("Response should be 'error_response', {}", err);
            //     }
            // };
        }
        Err(err) => {
            panic!("error: {}", err);
        }
    };

    tokio::time::sleep(Duration::from_secs(1)).await;

    let dummy_tx_hash = tc_dummy.get_tx_hash();

    let is_contain = machine
        .blockchain
        .dist_ledger
        .apis
        .tx_pool_contains(&dummy_tx_hash)
        .await;

    assert_eq!(true, is_contain);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_if_send_transaction_puts_false_tx_into_tx_pool() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let tc_dummy = PourTxCandidate::new_dummy_m1_to_p3_p4();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;
    let _rpc_server = tokio::spawn(async move { rpc.run().await });
    let client = Client::new();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/send_pour_tx",
            rpc_socket_addr.port()
        );
        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .body(Body::from(format!(
            r#"
                    {{
                        "pi": {:?},
                        "author_sig": "{}",
                        "created_at": "{}",
                        "data": {:?},
                        "ctr_addr": {:?},
                        "sn_1": {:?},
                        "sn_2": {:?},
                        "cm_1": {:?},
                        "cm_2": {:?},
                        "merkle_rt": {:?}                        
                    }}
                "#,
            tc_dummy.pi,
            tc_dummy.author_sig,
            tc_dummy.created_at,
            tc_dummy.data,
            tc_dummy.ctr_addr,
            tc_dummy.sn_1,
            tc_dummy.sn_2,
            tc_dummy.cm_1,
            tc_dummy.cm_2,
            tc_dummy.merkle_rt,
        )))
        .expect("request builder should be made");

    let _res = match client.request(req).await {
        Ok(res) => {
            let body = hyper::body::aggregate(res)
                .await
                .expect("body should be parsed");

            // let _: JsonResponse = match serde_json::from_reader(body.reader()) {
            //     Ok(e) => {
            //         log::info!("log info dbg {:?}", e);
            //         e
            //     }
            //     Err(err) => {
            //         panic!("Response should be 'error_response', {}", err);
            //     }
            // };
        }
        Err(err) => {
            panic!("error: {}", err);
        }
    };

    tokio::time::sleep(Duration::from_secs(1)).await;

    let false_tx_hash = String::from("false_tx");
    let is_contain = machine
        .blockchain
        .dist_ledger
        .apis
        .tx_pool_contains(&false_tx_hash)
        .await;

    assert_eq!(false, is_contain);
}
