use super::utils::{self, TestContext};
use crate::{rpc::routes::v0::GetNodeStatusResponse, tests::SaksahaTestUtils};
use hyper::{Body, Client, Method, Request, Uri};
use sak_credential::CredentialProfile;
use sak_rpc_interface::{JsonRequest, JsonResponse};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_request_correct_status() {
    let test_credential_1 = CredentialProfile::test_1();

    SaksahaTestUtils::init_test(&[&test_credential_1.public_key_str]);

    let TestContext {
        rpc,
        rpc_socket_addr,
        ..
    } = utils::make_test_context(test_credential_1.secret, test_credential_1.public_key_str).await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let uri: Uri = {
        let u = format!(
            "http://localhost:{}/apis/v0/get_status",
            rpc_socket_addr.port()
        );

        u.parse().expect("URI should be made")
    };

    let body = {
        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_status".to_string(),
            params: None,
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

    let resp = client.request(req).await.unwrap();

    let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    match serde_json::from_slice::<JsonResponse<GetNodeStatusResponse>>(&b) {
        Ok(b) => match b.result {
            Some(v) => {
                println!("Successfully get status : {:?}", v);
            }
            _ => panic!("Failed to get status"),
        },
        Err(err) => panic!("Failed to get response : {:?}", err),
    };
}
