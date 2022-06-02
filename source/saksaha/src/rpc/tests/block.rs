use super::utils::test_utils;

#[cfg(test)]
mod test_suite {
    use super::*;
    use crate::blockchain::ledger::for_test;
    use crate::blockchain::{BlockValue, Hash};
    use hyper::body::HttpBody;
    use hyper::{Body, Client, Method, Request, Uri};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_get_block() {
        test_utils::init();

        let (rpc, rpc_socket_addr, machine) = test_utils::make_rpc().await;

        let _rpc_server = tokio::spawn(async move { rpc.run().await });

        let client = Client::new();

        let block_value = BlockValue {
            tx_pool: vec![String::from("1"), String::from("2")],
            sig_vec: vec![String::from("1"), String::from("2")],
            created_at: String::from(""),
            height: String::from(""),
        };

        let block_hash = {
            let block_hash = match machine
                .blockchain
                .ledger
                .write_block(block_value.clone())
                .await
            {
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
            .body(Body::from(block_hash.hash))
            .expect("request builder should be made");

        match hyper::body::to_bytes(req.into_body()).await {
            Ok(b) => {
                let body_bytes_vec = b.to_vec();
                let _vh = match std::str::from_utf8(&body_bytes_vec) {
                    Ok(b) => {
                        let hash = &Hash {
                            hash: b.to_string(),
                        };
                        let _vht = match machine.get_block(hash).await {
                            Ok(block) => {
                                println!("{:?}", block);
                                assert_eq!(
                                    &block.tx_pool,
                                    &block_value.tx_pool
                                );
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
}
