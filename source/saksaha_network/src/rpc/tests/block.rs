use super::utils;
use sak_rpc_interface::{JsonRequest,JsonResponse};
use crate::rpc::routes::v0::{ GetBlockResponse, GetBlockListResponse};
use hyper::{Body, Client, Method, Request, Uri};

#[tokio::test(flavor = "multi_thread")]
async fn test_call_get_block_with_good_params() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let block_candidate_same = utils::make_dummy_tx_pour_block();

    let original_block_hash = {
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

        block_hash.unwrap()
    };

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let params = 
r#"
{
    "block_hash": "973f486c42f67e8520367a46f1a13caf969224d99d1b2f02943c6d926b7bc04b"
}
"#.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_block".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

        println!("[+] request body str (for debugging): {:#?}", str);

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let json_response =
        serde_json::from_slice::<JsonResponse<GetBlockResponse>>(&b).unwrap();

    let result = json_response.result.unwrap();

    let block_acquired = result.block.unwrap();

    assert_eq!(block_acquired.get_block_hash(), &original_block_hash);
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_call_get_block_with_wrong_params() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });

    let client = Client::new();

    let block_candidate = utils::make_dummy_tx_pour_block();

    let original_block_hash = {
        let block_hash = match machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(Some(block_candidate))
            .await
        {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        block_hash.unwrap()
    };

    let uri: Uri = {
        let u = format!("http://localhost:{}", rpc_socket_addr.port());

        u.parse().expect("URI should be made")
    };

    let body = {
        let params = 
r#"
973f486c42f67e8520367a46f1a13caf969224d99d1b2f02943c6d926b7bc04b
"#.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_block".to_string(),
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

        println!("[+] request body str (for debugging): {:#?}", str);

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let resp_str = std::str::from_utf8(&b).unwrap();

    println!("[+] response (for debugging): {}", resp_str);

    let json_response =
        serde_json::from_slice::<JsonResponse<GetBlockResponse>>(&b).unwrap();

    println!("[+] resp struct: {:?}", json_response);

    let result = json_response.result.unwrap();

    let block_acquired = result.block.unwrap();

    println!(
        "[+] block hash (from rpc response) : {:?}",
        block_acquired.get_block_hash(),
    );

    assert_eq!(block_acquired.get_block_hash(), &original_block_hash);

    //
}


#[tokio::test(flavor = "multi_thread")]
async fn test_call_get_block_list() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (rpc, rpc_socket_addr, machine) = utils::make_test_context().await;

    tokio::spawn(async move { rpc.run().await });



    let mut block_hashes: Vec<String> = Vec::new();

    for _ in 0..10 {
        let block_candidate = utils::make_dummy_tx_pour_block();

        let block_hash = {
            let block_hash = match machine
                .blockchain
                .dist_ledger
                .apis
                .write_block(Some(block_candidate))
                .await
            {
                Ok(v) => v,
                Err(err) => panic!("Failed to write dummy block, err: {}", err),
            };

            block_hash.unwrap()
        };

        block_hashes.push(block_hash)
    }
    println!("[+] block_hash: {:#?}", block_hashes);

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };
    println!("[+] URI: {:#?}", uri);

    let body = {
        let params = 
r#"
{
    "block_height": 3
}
"#.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_block_list".to_string(),
            params:Some(params), 
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request).unwrap();

        Body::from(str)
    };


    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");
    println!("[+] Request: {:#?}", req);

    let client = Client::new();

    let response = client.request(req).await.unwrap();

    let response_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();

    let json_response =
        serde_json::from_slice::<JsonResponse<GetBlockListResponse>>(&response_bytes).unwrap();
    println!("[+] json_response: {:#?}", json_response);

    let result = json_response.result.unwrap();
    println!("[+] result: {:#?}", result);

    let block_hashes_acquired = result.block_list;
    println!("[+] block hashes acquired: {:#?}", block_hashes_acquired);


    println!("----");
    println!("[+] original block hashes: {:#?}", block_hashes);
    println!("[+] index starts at: {}, acquired block hashes: {:#?}", 3, block_hashes_acquired);

    assert_eq!(&Some(block_hashes[2].clone()), &block_hashes_acquired[0]);
    assert_eq!(&Some(block_hashes[3].clone()), &block_hashes_acquired[1]);
    assert_eq!(&Some(block_hashes[4].clone()), &block_hashes_acquired[2]);
    assert_eq!(&Some(block_hashes[5].clone()), &block_hashes_acquired[3]);
    assert_eq!(&Some(block_hashes[6].clone()), &block_hashes_acquired[4]);
    assert_eq!(&Some(block_hashes[7].clone()), &block_hashes_acquired[5]);
    assert_eq!(&Some(block_hashes[8].clone()), &block_hashes_acquired[6]);
    assert_eq!(&Some(block_hashes[9].clone()), &block_hashes_acquired[7]);
    assert_eq!(&None, &block_hashes_acquired[8]);
    assert_eq!(&None, &block_hashes_acquired[9]);


}
