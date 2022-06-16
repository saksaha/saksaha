use super::utils::test_utils;

#[cfg(test)]
mod test_suite {
    use super::*;
    use hyper::{Body, Client, Method, Request, Uri};
    use sak_types::Block;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_get_block() {
        test_utils::init();

        let (rpc, rpc_socket_addr, machine) = test_utils::make_rpc().await;

        let rpc_server = tokio::spawn(async move { rpc.run().await });

        let client = Client::new();

        let block_value = Block::new(
            String::from("1"),
            vec![String::from("1"), String::from("2")],
            vec![String::from("1"), String::from("2")],
            String::from(""),
            String::from(""),
        );

        let block_hash = {
            let block_hash = match machine
                .blockchain
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
                "http://localhost:{}/apis/v0/get_block",
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
                let _vh = match std::str::from_utf8(&body_bytes_vec) {
                    Ok(b) => {
                        let hash = &b.to_string();
                        let _vht = match machine.get_block(hash).await {
                            Ok(block) => {
                                println!("{:?}", block);

                                assert_eq!(
                                    &block.get_tx_hashes(),
                                    &block_value.get_tx_hashes(),
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
