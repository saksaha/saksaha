use crate::rpc::routes::v0::{SendTxRequest, SendTxResponse};
use crate::{
    credential::WalletCredential, db::WalletDB, rpc::RPC, wallet::Wallet, Config, CredentialManager,
};
use envelope_contract::{request_type, Channel, OpenChParams};
use envelope_term::ENVELOPE_CTR_ADDR;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrRequest, CtrRequestData};
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use std::sync::Arc;

pub(crate) struct MockWalletContext {
    pub wallet: Arc<Wallet>,
    pub rpc: RPC,
    pub acc_addr: String,
}
pub(crate) async fn mock_wallet_context() -> MockWalletContext {
    let config = Config::new(&Some("dev_local_1".to_string())).unwrap();

    let public_key = config.public_key.clone().unwrap();
    let secret = config.secret.clone().unwrap();

    let wallet_credential = WalletCredential::load(&public_key, &secret).unwrap();

    let credential_manager = CredentialManager::init(wallet_credential).unwrap();

    let acc_addr = credential_manager.get_credential().acc_addr.clone();

    let wallet_db = WalletDB::init(&credential_manager.get_credential(), true).unwrap();

    let wallet = {
        let w = Wallet::init(credential_manager, wallet_db, config)
            .await
            .unwrap();

        Arc::new(w)
    };

    let rpc = RPC::init(Some(36612), wallet.clone()).await.unwrap();

    MockWalletContext {
        wallet,
        rpc,
        acc_addr,
    }
}

pub(crate) async fn mock_node_server() {}

pub(crate) async fn mock_credential_manager() -> CredentialManager {
    // dev_local_1
    let public_key = String::from(
        "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
    );

    let secret = String::from(
        "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
    );

    let wallet_credential = WalletCredential::load(&public_key, &secret).unwrap();

    let m = CredentialManager::init(wallet_credential).unwrap();

    m
}

pub(crate) async fn mock_send_pour_tx(
    rpc_port: u16,
    acc_addr: &String,
) -> JsonResponse<SendTxResponse> {
    let client = Client::new();

    let uri: Uri = {
        let u = format!("http://localhost:{}", rpc_port);

        u.parse().expect("URI should be made")
    };

    //
    let body = {
        let (_, eph_pub_key) = SakKey::generate();

        let channel = Channel::new(
            // ch_id
            "ch_id".to_string(),
            // eph_key
            sak_crypto::encode_hex(&eph_pub_key.to_encoded_point(false).to_bytes()),
            // initiator_pk
            "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
                .to_string(),
            // participants
            vec![
                "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
                    .to_string(),
                "\
                042c8d005bd935597117181d8ceceaef\
                6d1162de78c3285689d0c36c6170634c\
                124f7b9b911553a1f483ec565c199ea2\
                9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
                    .to_string(),
            ],
        )
        .unwrap();

        let open_ch_params = OpenChParams {
            dst_pk: "\
                042c8d005bd935597117181d8ceceaef\
                6d1162de78c3285689d0c36c6170634c\
                124f7b9b911553a1f483ec565c199ea2\
                9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
                .to_string(),
            open_ch: channel,
        };

        let args = serde_json::to_vec(&open_ch_params).unwrap();

        let ctr_request_data = CtrRequestData {
            req_type: request_type::SEND_MSG.to_string(),
            args,
            ctr_call_type: sak_contract_std::CtrCallType::Execute,
        };

        let send_tx_req = SendTxRequest {
            acc_addr: acc_addr.clone(),
            ctr_addr: ENVELOPE_CTR_ADDR.to_string(),
            ctr_request_data,
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

    let json_response = serde_json::from_slice::<JsonResponse<SendTxResponse>>(&b).unwrap();

    json_response
}

pub(crate) async fn mock_update_coin_status(
    rpc_port: u16,
    acc_addr: &String,
) -> JsonResponse<String> {
    let client = Client::new();

    let uri: Uri = {
        let u = format!("http://localhost:{}", rpc_port);

        u.parse().expect("URI should be made")
    };

    let body = {
        let ctr_request_data = CtrRequestData {
            req_type: request_type::SEND_MSG.to_string(),
            args: vec![],
            ctr_call_type: sak_contract_std::CtrCallType::Execute,
        };

        let send_tx_req = SendTxRequest {
            acc_addr: acc_addr.clone(),
            ctr_addr: ENVELOPE_CTR_ADDR.to_string(),
            ctr_request_data,
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

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b).unwrap();

    json_response
}
