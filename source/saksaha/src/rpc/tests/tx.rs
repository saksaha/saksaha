use super::utils::test_utils;

#[cfg(test)]
mod test_suite {
    use super::*;
    use crate::blockchain::{ledger_for_test, Hashable};
    use crate::rpc::response::{ErrorResponse, SuccessResponse};
    use hyper::body::Buf;
    use hyper::{Body, Client, Method, Request, Uri};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_send_wrong_transaction() {
        test_utils::init();

        let (rpc, rpc_socket_addr, _) = test_utils::make_rpc().await;

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
            let ledger = blockchain.ledger;
            let dummy_tx_val = test_utils::make_dummy_value();

            let old_tx_hash =
                (&dummy_tx_val).get_hash().expect("fail to get hash");

            ledger_for_test::delete_tx(&ledger, &old_tx_hash)
                .expect("Tx should be deleted");

            let _tx_hash = ledger
                .write_tx(dummy_tx_val)
                .await
                .expect("Tx should be written");
        }

        let (rpc, rpc_socket_addr, _) = test_utils::make_rpc().await;

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

        {
            let blockchain = test_utils::make_blockchain().await;
            let ledger = blockchain.ledger;
            let dummy_tx_val = test_utils::make_dummy_value();

            let old_tx_hash =
                (&dummy_tx_val).get_hash().expect("fail to get hash");

            ledger_for_test::delete_tx(&ledger, &old_tx_hash)
                .expect("Tx should be deleted");

            let tx_hash = ledger
                .write_tx(dummy_tx_val)
                .await
                .expect("Tx should be written");

            assert_eq!(old_tx_hash, tx_hash);
        }

        let (rpc, rpc_socket_addr, _) = test_utils::make_rpc().await;

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
            .body(Body::from(
                r#"{"hash": "8082e05e6adf824f9c024e64f9fb2f6b04bbf02d455f69807b5bc58976025cd0"}"#))
            .expect("request builder should be made");

        println!("{:?}", req);

        let _res = match client.request(req).await {
            Ok(res) => {
                let body = hyper::body::aggregate(res)
                    .await
                    .expect("body should be parsed");

                let _: SuccessResponse =
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
}
