#[cfg(test)]
mod test {
    use crate::blockchain::ledger::{self, for_test, Hashing};
    use crate::p2p::P2PState;
    use crate::{rpc::response::{ErrorResponse, SuccessResponse, SuccessResult, ErrorResult}, blockchain::BlockValue};
    use crate::rpc::RPC;
    use crate::{
        blockchain::{Blockchain, BlockchainArgs, TxValue},
        machine::Machine,
        rpc::RPCArgs,
    };
    use hyper::{body::Buf, StatusCode};
    use hyper::{Body, Client, Method, Request, Uri};
    use p2p_discovery::{Discovery, DiscoveryArgs};
    use p2p_identity::addr::{AddrStatus, UnknownAddr};
    use p2p_identity::identity::P2PIdentity;
    use p2p_peer::PeerTable;
    use std::{net::SocketAddr, sync::Arc};
    use tokio::net::TcpListener;

    const app_prefix: &str = "test";

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_rpc() -> (RPC, TcpListener, SocketAddr, Arc<Machine>) {
        let (rpc_socket, rpc_socket_addr) =
            utils_net::bind_tcp_socket(Some(12345))
                .await
                .expect("rpc socket should be initialized");

        let blockchain = {
            let blockchain_args = BlockchainArgs {
                app_prefix: "test".to_string(),
            };

            Blockchain::init(blockchain_args).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let secret = String::from(
            "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
        );

        let public_key_str = String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35521,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
            )),
            status: AddrStatus::Initialized,
        }];

        let p2p_identity = {
            let id = P2PIdentity::new(secret, public_key_str).unwrap();
            Arc::new(id)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            p2p_identity: p2p_identity.clone(),
            disc_port: Some(35521),
            p2p_port: 1,
            bootstrap_addrs,
        };

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");

            Arc::new(d)
        };

        let p2p_state = {
            let s = P2PState {
                p2p_identity: p2p_identity.clone(),
                p2p_port: 35521,
                rpc_port: 12345,
                p2p_peer_table: p2p_peer_table.clone(),
                p2p_discovery: p2p_discovery.clone(),
            };

            Arc::new(s)
        };

        let rpc = {
            let rpc_args = RPCArgs {
                machine: machine.clone(),
                p2p_state: p2p_state.clone(),
            };

            RPC::init(rpc_args).expect("RPC should be initialized")
        };

        (rpc, rpc_socket, rpc_socket_addr, machine)
    }

    async fn make_blockchain() -> Blockchain {
        let blockchain_args = BlockchainArgs {
            app_prefix: String::from("test"),
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_value() -> TxValue {
        TxValue {
            pi: String::from("0x111"),
            sig_vec: String::from("0x1111"),
            created_at: String::from("1346546123"),
            data: String::from("one"),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_send_wrong_transaction() {
        init();

        let (rpc, rpc_socket, rpc_socket_addr, _) = make_rpc().await;

        let _rpc_server =
            tokio::spawn(
                async move { rpc.run(rpc_socket, rpc_socket_addr).await },
            );

        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v1/send_transaction",
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
        init();

        {
            let blockchain = make_blockchain().await;
            let ledger = blockchain.ledger;
            let dummy_tx_val = make_dummy_value();

            let old_tx_hash = (&dummy_tx_val).get_hash().expect("fail to get hash");

            for_test::delete_tx(&ledger, &old_tx_hash.hash)
                .expect("Tx should be deleted");

            let _tx_hash = ledger
                .write_tx(dummy_tx_val)
                .await
                .expect("Tx should be written");
        }

        let (rpc, rpc_socket, rpc_socket_addr, _) = make_rpc().await;

        let _rpc_server =
            tokio::spawn(
                async move { rpc.run(rpc_socket, rpc_socket_addr).await },
            );

        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v1/get_transaction",
                rpc_socket_addr.port()
            );

            u.parse().expect("URI should be made")
        };

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(r#"{"hash": "1q2w3e4r"}"#))
            .expect("request builder should be made");

        let res = match client.request(req).await {
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
        init();

        {
            let blockchain = make_blockchain().await;
            let ledger = blockchain.ledger;
            let dummy_tx_val = make_dummy_value();

            let old_tx_hash = (&dummy_tx_val).get_hash().expect("fail to get hash");

            for_test::delete_tx(&ledger, &old_tx_hash.hash)
                .expect("Tx should be deleted");

            let tx_hash = ledger
                .write_tx(dummy_tx_val)
                .await
                .expect("Tx should be written");

            assert_eq!(old_tx_hash.hash, tx_hash);
        }

        let (rpc, rpc_socket, rpc_socket_addr, _) = make_rpc().await;

        let _rpc_server =
            tokio::spawn(
                async move { rpc.run(rpc_socket, rpc_socket_addr).await },
            );

        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v1/get_transaction",
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_get_block() {
        init();

        let (rpc, rpc_socket, rpc_socket_addr, machine) = make_rpc().await;

        let _rpc_server =
            tokio::spawn(
                async move { rpc.run(rpc_socket, rpc_socket_addr).await },
            );

        let client = Client::new();

        let block_value = BlockValue {
                tx_pool: vec![String::from("1"), String::from("2")],
                sig_vec:vec![String::from("1"), String::from("2")],
                created_at: String::from(""),
                height: String::from(""),
            };

        let block_hash = {
            let block_hash = match machine.blockchain.ledger.write_block(block_value.clone()).await {
                Ok(v) => v,
                Err(err) => panic!("Failed to write dummy block, err: {}", err),
            };

            block_hash
        };

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v1/get_block",
                rpc_socket_addr.port()
            );

            u.parse().expect("URI should be made")
        };

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(block_hash))
            .expect("request builder should be made");

        match hyper::body::to_bytes(req.into_body()).await {
            Ok(b) => {
                let body_bytes_vec = b.to_vec();
            let vh = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let vht = match machine.get_block(&b.to_string()).await {
                        Ok(block) => {
                            println!("{:?}", block);
                            assert_eq!(&block.tx_pool, &block_value.tx_pool);
                        },
                        Err(err) => panic!(),
                };
            },
                Err(err) => panic!(),
        };
            },
            Err(err) => panic!(),
        }
    }
}
