use super::utils::{self, TestContext};
use crate::{
    rpc::routes::v0::{GetCmIdxRequest, GetCmIdxResponse},
    tests::SaksahaTestUtils,
};
use hyper::{Body, Client, Method, Request, Uri};
use sak_credential::CredentialProfile;
use sak_rpc_interface::{JsonRequest, JsonResponse};
use sak_types::{BlockCandidate, MintTxCandidate, PourTxCandidate, Tx, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_handle_get_cm_idx() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test"]);

    let test_credential_1 = CredentialProfile::test_1();

    SaksahaTestUtils::init_test(&[&test_credential_1.public_key_str]);

    let (expected_tx_hash, cms) = {
        let blockchain =
            utils::make_blockchain(&test_credential_1.secret, &test_credential_1.public_key_str)
                .await;

        // let dummy_tx = sak_types::mock_pour_tc_m1_to_p3_p4();
        let dummy_tx = sak_types::mock_pour_tc_1();

        let cms = dummy_tx.get_cms().clone();

        println!("power: {:?}", cms);

        let old_tx_hash = (&dummy_tx).get_tx_hash();

        let dist_ledger = blockchain.dist_ledger;

        let bc = Some(BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![dummy_tx.clone()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: format!("{}", 0),
        });

        dist_ledger.apis.write_block(bc).await.unwrap();

        let tx = dist_ledger
            .apis
            .get_tx(&old_tx_hash.clone())
            .await
            .expect("Tx should be exist")
            .unwrap();

        let tx_hash = tx.get_tx_hash().clone();

        assert_eq!(tx_hash, *old_tx_hash);

        (old_tx_hash.clone(), cms)
    };

    let TestContext {
        rpc,
        rpc_socket_addr,
        ..
    } = utils::make_test_context(
        test_credential_1.secret.to_string(),
        test_credential_1.public_key_str.to_string(),
    )
    .await;

    let client = Client::new();

    tokio::spawn(async move { rpc.run().await });

    let uri: Uri = {
        let u = format!("http://localhost:{}/apis/v0", rpc_socket_addr.port());

        u.parse().expect("URI should be made")
    };

    let body = {
        let send_req = GetCmIdxRequest { cm: cms[0] };

        let params = serde_json::to_string(&send_req)
            .unwrap()
            .as_bytes()
            .to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_cm_idx".to_string(),
            params: Some(params),
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

    let json_response = serde_json::from_slice::<JsonResponse<GetCmIdxResponse>>(&b).unwrap();

    let tx_from_res = json_response.result.unwrap();

    let res = tx_from_res.cm_idx;
    println!("powww, {:?}", res);

    // assert_eq!(&expected_tx_hash, tx_hash_from_res);
}
