use super::utils::test_utils;

#[cfg(test)]
mod test_suite {
    use crate::{blockchain::GenesisBlock, rpc::response::JsonResponse};

    use super::*;
    use hyper::body::Buf;
    use hyper::{Body, Client, Method, Request, Uri};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_contract() {
        test_utils::init();

        let (rpc, rpc_socket_addr, _machine) = test_utils::make_rpc().await;

        let client = Client::new();

        tokio::spawn(async move { rpc.run().await });

        let genesis_block = GenesisBlock::create();
        let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v0/call_contract",
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
                        "ctr_addr": {:?},
                        "ctr_fn": {:?}
                    }}
                "#,
                validator_ctr_addr,
                "get_validator".to_string(),
            )))
            .expect("request builder should be made");

        match client.request(req).await {
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
            Err(_) => {
                println!("4");
                panic!()
            }
        }
    }
}
