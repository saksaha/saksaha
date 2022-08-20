use std::time::Duration;

use super::utils::{self, TestContext};
use crate::{
    routes::v0::update_coin_status,
    rpc::routes::v0::{
        GetBalanceRequest, GetBalanceResponse, SendTxRequest, SendTxResponse,
    },
};
use envelope_contract::request_type;
use envelope_term::ENVELOPE_CTR_ADDR;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::CtrRequest;
use sak_rpc_interface::{JsonRequest, JsonResponse};
pub(crate) const RPC_PORT: u16 = 36612;

#[tokio::test(flavor = "multi_thread")]
async fn test_send_tx() {
    sak_test_utils::init_test_log();

    let test_credential = utils::make_test_credential().await;
    let acc_addr = &test_credential.get_credential().acc_addr;

    let client = Client::new();

    let uri: Uri = {
        let u = format!("http://localhost:{}", RPC_PORT);

        u.parse().expect("URI should be made")
    };

    let body = {
        let ctr_request = CtrRequest {
            req_type: request_type::SEND_MSG.to_string(),
            args: vec![],
            ctr_call_type: sak_contract_std::CtrCallType::Execute,
        };

        let send_tx_req = SendTxRequest {
            acc_addr: acc_addr.clone(),
            ctr_addr: ENVELOPE_CTR_ADDR.to_string(),
            ctr_request,
        };

        let params = serde_json::to_vec(&send_tx_req).unwrap();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_pour_tx".to_string(),
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

    let json_response =
        serde_json::from_slice::<JsonResponse<SendTxResponse>>(&b).unwrap();

    let _result = json_response.result.ok_or("empty").unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_send_tx_twice() {
    sak_test_utils::init_test_log();

    // let test_credential = utils::make_test_credential().await;

    let test_context = utils::mock_test_context().await;

    let TestContext {
        wallet,
        rpc,
        acc_addr,
    } = test_context;

    tokio::spawn(async move { rpc.run().await });

    // let acc_addr = &test_credential.get_credential().acc_addr;

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("\t111111111 {:?}", balance);

        utils::send_msg_for_test(&acc_addr).await;
        utils::update_coin_status(&acc_addr).await;
    }

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("222222222 {:?}", balance);

        utils::send_msg_for_test(&acc_addr).await;
        utils::update_coin_status(&acc_addr).await;
    }
}
