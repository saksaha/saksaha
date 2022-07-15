use super::utils;
use hyper::{Body, Client, Method, Request, Uri};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_get_block() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    let _rpc_server = tokio::spawn(async move { rpc.run().await });

    let _client = Client::new();

    let block_candidate = utils::make_dummy_genesis_block();
    let block_candidate_same = utils::make_dummy_genesis_block();

    // let (block_value, _) = block_candidate.upgrade(None, None, None);

    let block_hash = {
        let block_hash = match machine
            .blockchain
            .dist_ledger
            .write_block(Some(block_candidate_same))
            .await
        {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        block_hash
    };

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_block",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(Body::from(block_hash.unwrap()))
        .expect("request builder should be made");

    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _vh = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let hash = &b.to_string();
                    let _vht =
                        match machine.blockchain.dist_ledger.get_block(hash) {
                            Ok(block) => {
                                println!("{:?}", block);

                                // TODO compare some values here!
                                // assert_eq!(
                                //     &block.unwrap().get_tx_hashes(),
                                //     &block_value.get_tx_hashes(),
                                // );
                            }
                            Err(_err) => panic!(),
                        };
                }
                Err(_err) => panic!(),
            };
        }
        Err(_err) => panic!(),
    }
}
