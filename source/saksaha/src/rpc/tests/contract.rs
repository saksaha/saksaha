use super::utils;
use crate::rpc::routes::v0::CallContractBody;
use crate::{blockchain::GenesisBlock, rpc::response::JsonResponse};
use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_call_contract() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, _machine) = utils::make_test_context().await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let genesis_block = GenesisBlock::create().unwrap();
    let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/call_contract",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let request = {
        let ctr_addr = validator_ctr_addr;
        let req = CtrRequest {
            req_type: "get_validator".to_string(),
            arg: HashMap::with_capacity(10),
            ctr_call_type: CtrCallType::Query,
        };

        let call_ctr_body = CallContractBody { ctr_addr, req };

        call_ctr_body
    };

    let body_string = serde_json::to_string(&request).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .body(Body::from(body_string))
        .expect("request builder should be made");

    match client.request(req).await {
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
        Err(_) => {
            println!("4");
            panic!()
        }
    }
}
