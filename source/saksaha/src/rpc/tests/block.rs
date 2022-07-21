use std::io::Read;

use crate::rpc::response::JsonResponse;

use super::utils;
use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Uri};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_get_block() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    let _rpc_server = tokio::spawn(async move { rpc.run().await });

    let _client = Client::new();

    let block_candidate_same = utils::make_dummy_tx_pour_block();

    let block_hash = {
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
        .body(Body::from(block_hash.clone().unwrap()))
        .expect("request builder should be made");

    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _vh = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let hash = &b.to_string();
                    println!("block hash : {:?}", hash);
                    let _vht = match machine
                        .blockchain
                        .dist_ledger
                        .apis
                        .get_block(hash)
                    {
                        Ok(block) => {
                            // println!("{:?}", &block);
                            let block = block.unwrap();
                            let block_hash_from_get_block =
                                block.get_block_hash();

                            let block_hash_expected = block_hash.unwrap();

                            assert_eq!(
                                block_hash_expected,
                                block_hash_from_get_block.to_string()
                            );
                        }
                        Err(_err) => panic!("error : {:?}", _err),
                    };
                }
                Err(_err) => panic!(),
            };
        }
        Err(_err) => panic!(),
    }
}
