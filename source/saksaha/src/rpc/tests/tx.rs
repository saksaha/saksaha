use super::utils::test_utils;

#[cfg(test)]
mod test_suite {
    use super::*;
    use crate::blockchain::{self, Blockchain};
    use crate::rpc::response::{ErrorResponse, JsonResponse, SuccessResponse};
    use hyper::body::Buf;
    use hyper::{Body, Client, Method, Request, Uri};
    use sak_dist_ledger::DistLedger;
    use sak_types::{Hashable, PourTxCandidate, TxCandidate};
    use std::time::Duration;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_send_wrong_transaction() {
        test_utils::init();

        let (rpc, rpc_socket_addr, _) = test_utils::make_test_context().await;

        let _rpc_server = tokio::spawn(async move { rpc.run().await });

        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v0/send_transaction",
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

                let _: ErrorResponse =
                    match serde_json::from_reader(body.reader()) {
                        Ok(e) => e,
                        Err(err) => {
                            panic!(
                                "Response should be 'error_response', {}",
                                err
                            );
                        }
                    };
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        };
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_request_wrong_transaction_hash() {
        test_utils::init();

        {
            let blockchain = test_utils::make_blockchain().await;

            let dummy_tx = TxCandidate::new_dummy_pour_1();

            let old_tx_hash = (&dummy_tx).get_tx_hash();

            let dist_ledger = blockchain.dist_ledger;

            dist_ledger
                .delete_tx(&old_tx_hash)
                .expect("Tx should be deleted");

            let _tx_hash = dist_ledger
                .send_tx(dummy_tx)
                .await
                .expect("Tx should be written");
        }

        let (rpc, rpc_socket_addr, _) = test_utils::make_test_context().await;

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

                let _: ErrorResponse =
                    match serde_json::from_reader(body.reader()) {
                        Ok(e) => {
                            log::info!("{:?}", e);
                            e
                        }
                        Err(err) => {
                            panic!(
                                "Response should be 'error_response', {}",
                                err
                            );
                        }
                    };
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        };
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_request_correct_transaction_hash() {
        test_utils::init();

        let tx_hash = {
            let blockchain = test_utils::make_blockchain().await;

            let dummy_tx = TxCandidate::new_dummy_pour_1();

            let old_tx_hash = (&dummy_tx).get_tx_hash();

            let dist_ledger = blockchain.dist_ledger;

            dist_ledger
                .delete_tx(&old_tx_hash)
                .expect("Tx should be deleted");

            dist_ledger
                .send_tx(dummy_tx.clone())
                .await
                .expect("Tx should be written");

            let tx = dist_ledger
                .get_tx(&old_tx_hash.clone())
                .await
                .expect("Tx should be exist")
                .unwrap();

            let tx_hash = tx.get_tx_hash().clone();

            assert_eq!(tx_hash, *old_tx_hash);
            tx_hash
        };

        let (rpc, rpc_socket_addr, _) = test_utils::make_test_context().await;

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

        println!("Request: {:?}", req);

        let _res = match client.request(req).await {
            Ok(res) => {
                let body = hyper::body::aggregate(res)
                    .await
                    .expect("body should be parsed");

                let _: JsonResponse =
                    match serde_json::from_reader(body.reader()) {
                        Ok(e) => {
                            log::info!("{:?}", e);
                            e
                        }
                        Err(err) => {
                            panic!(
                                "Response should be 'error_response', {}",
                                err
                            );
                        }
                    };
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        };
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_if_send_transaction_puts_tx_into_tx_pool() {
        test_utils::init();

        let tc_dummy = PourTxCandidate::new_dummy_1();

        let (rpc, rpc_socket_addr, machine) =
            test_utils::make_test_context().await;
        let _rpc_server = tokio::spawn(async move { rpc.run().await });
        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v0/send_transaction",
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
                        "pi": "{:?}",
                        "signature": "{}",
                        "created_at": "{}",
                        "data": {:?},
                        "contract": {:?}
                    }}
                "#,
                tc_dummy.pi,
                tc_dummy.author_sig,
                tc_dummy.created_at,
                tc_dummy.data,
                tc_dummy.ctr_addr,
            )))
            .expect("request builder should be made");

        let _res = match client.request(req).await {
            Ok(res) => {
                let body = hyper::body::aggregate(res)
                    .await
                    .expect("body should be parsed");
                let _: JsonResponse =
                    match serde_json::from_reader(body.reader()) {
                        Ok(e) => {
                            log::info!("log info dbg {:?}", e);
                            e
                        }
                        Err(err) => {
                            panic!(
                                "Response should be 'error_response', {}",
                                err
                            );
                        }
                    };
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
            .tx_pool_contains(&dummy_tx_hash)
            .await;

        assert_eq!(true, is_contain);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_if_send_transaction_puts_false_tx_into_tx_pool() {
        test_utils::init();

        let tc_dummy = PourTxCandidate::new_dummy_1();

        let (rpc, rpc_socket_addr, machine) =
            test_utils::make_test_context().await;
        let _rpc_server = tokio::spawn(async move { rpc.run().await });
        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v0/send_transaction",
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
                        "pi": "{:?}",
                        "signature": "{}",
                        "created_at": "{}",
                        "data": {:?},
                        "contract": {:?}
                    }}
                "#,
                tc_dummy.pi,
                tc_dummy.author_sig,
                tc_dummy.created_at,
                tc_dummy.data,
                tc_dummy.ctr_addr,
            )))
            .expect("request builder should be made");

        let _res = match client.request(req).await {
            Ok(res) => {
                let body = hyper::body::aggregate(res)
                    .await
                    .expect("body should be parsed");
                let _: JsonResponse =
                    match serde_json::from_reader(body.reader()) {
                        Ok(e) => {
                            log::info!("log info dbg {:?}", e);
                            e
                        }
                        Err(err) => {
                            panic!(
                                "Response should be 'error_response', {}",
                                err
                            );
                        }
                    };
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
            .tx_pool_contains(&false_tx_hash)
            .await;

        assert_eq!(false, is_contain);
    }
}
