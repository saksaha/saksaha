use crate::rpc::routes::v0::{SendTxRequest, SendTxResponse};
use crate::{
    credential::WalletCredential, db::WalletDB, rpc::RPC, wallet::Wallet,
    Config, CredentialManager,
};
use envelope_contract::{request_type, SendMsgParams};
use envelope_term::ENVELOPE_CTR_ADDR;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::CtrRequest;
use sak_rpc_interface::{JsonRequest, JsonResponse};
use std::sync::Arc;

pub(crate) const RPC_PORT: u16 = 36612;

pub(crate) struct TestContext {
    pub wallet: Arc<Wallet>,
    pub rpc: RPC,
    pub acc_addr: String,
}

pub(crate) async fn mock_test_context() -> TestContext {
    let config = Config::new(&Some("dev_local_1".to_string()), &None).unwrap();

    let public_key = config.public_key.clone().unwrap();
    let secret = config.secret.clone().unwrap();

    let wallet_credential =
        WalletCredential::load(&public_key, &secret).unwrap();

    let credential_manager =
        CredentialManager::init(wallet_credential).unwrap();

    let acc_addr = credential_manager.get_credential().acc_addr.clone();

    let wallet_db =
        WalletDB::init(&credential_manager.get_credential(), true).unwrap();

    let wallet = {
        let mut w = Wallet::init(credential_manager, wallet_db, config)
            .await
            .unwrap();

        Arc::new(w)
    };

    let rpc = RPC::init(Some(36612), wallet.clone()).await.unwrap();

    TestContext {
        wallet,
        rpc,
        acc_addr,
    }
}

pub(crate) async fn make_test_credential() -> CredentialManager {
    let public_key = String::from(
        "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
    );

    let secret = String::from(
        "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
    );

    let wallet_credential =
        WalletCredential::load(&public_key, &secret).unwrap();

    let m = CredentialManager::init(wallet_credential).unwrap();

    m
}

pub(crate) async fn send_msg_for_test(acc_addr: &String) {
    let client = Client::new();

    let uri: Uri = {
        let u = format!("http://localhost:{}", RPC_PORT);

        u.parse().expect("URI should be made")
    };

    let body = {
        let send_msg_params = SendMsgParams {
            ch_id: String::from("ch_0"),
            msg: String::from("hi"),
        };

        let args = serde_json::to_vec(&send_msg_params).unwrap();

        let ctr_request = CtrRequest {
            req_type: request_type::SEND_MSG.to_string(),
            args,
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

    let _json_response =
        serde_json::from_slice::<JsonResponse<SendTxResponse>>(&b).unwrap();
}

pub(crate) async fn update_coin_status(acc_addr: &String) -> String {
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
            method: "update_coin_status".to_string(),
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

    let _json_response =
        serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    "power".to_string()
    // json_response.result.unwrap()
}
