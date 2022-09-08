use super::utils::{self, MockWalletContext};
use envelope_contract::{request_type, Channel, OpenChParams};
use envelope_term::ENVELOPE_CTR_ADDR;
use sak_contract_std::CtrRequest;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_types::CoinRecord;

#[tokio::test(flavor = "multi_thread")]
async fn test_send_tx() {
    sak_test_utils::init_test_log();

    let acc_addr = {
        let credential_manager = utils::mock_credential_manager().await;

        credential_manager.get_credential().acc_addr.clone()
    };

    let mock_context = utils::mock_wallet_context().await;

    let rpc = mock_context.rpc;

    let rpc_port = rpc.get_rpc_port();

    tokio::spawn(async move { rpc.run().await });

    let json_response = utils::mock_send_pour_tx(rpc_port, &acc_addr).await;

    println!("response: {:#?}", json_response);

    json_response.result.unwrap();

    //
    let wallet = mock_context.wallet;

    let coin_manager = wallet.get_coin_manager().write().await;

    let coins = coin_manager.get_all_coins();

    println!("coins: {:#?}", coins);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update_coin_status() {
    sak_test_utils::init_test_log();

    let acc_addr = {
        let credential_manager = utils::mock_credential_manager().await;

        credential_manager.get_credential().acc_addr.clone()
    };

    let mock_context = utils::mock_wallet_context().await;

    let rpc = mock_context.rpc;

    let rpc_port = rpc.get_rpc_port();

    println!("rpc_port: {:?}", rpc_port);

    tokio::spawn(async move { rpc.run().await });

    let json_response =
        utils::mock_update_coin_status(rpc_port, &acc_addr).await;

    println!("json_response: {:#?}", json_response);

    log::warn!(
        "This test skip the step `unconfirmed to unused`\
        which need a running node"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_print_dummy_merkle_rt() {
    sak_test_utils::init_test_log();

    // let test_credential = utils::make_test_credential().await;

    let test_context = utils::mock_wallet_context().await;

    let wallet = test_context.wallet;

    let dummy_coin = CoinRecord::new_dummy();

    let dummy_auth_path = wallet.prepare_dummy_auth_path().await.unwrap();

    let dummy_merkle_rt = wallet
        .prepare_merkle_rt(&dummy_coin, dummy_auth_path.clone())
        .unwrap();

    // [247, 154, 75, 119, 90, 47, 200, 133, 182, 132, 225, 10, 46, 184, 117, 21, 34, 4, 99, 216, 220, 128, 7, 244, 99, 90, 167, 93, 251, 176, 236, 18]
    println!("dummy_merkle_rt: {:?}", dummy_merkle_rt);

    // [214, 107, 131, 229, 87, 169, 202, 14, 124, 201, 178, 160, 124, 64, 127, 131, 1, 79, 76, 17, 161, 60, 250, 110, 102, 175, 33, 193, 105, 88, 32, 70]
    println!("compute sn: {:?}", dummy_coin.compute_sn());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_send_tx_with_2_old_coins() {
    sak_test_utils::init_test_log();

    let MockWalletContext { wallet, .. } = utils::mock_wallet_context().await;

    let coin_manager_lock = wallet.get_coin_manager().write().await;

    let coins = coin_manager_lock.get_all_coins().unwrap();
    assert_eq!(coins.len(), 2);

    let (old_coin_1, sn_1, merkle_rt_1) = {
        let cm_idx = wallet.prepare_cm_idx(&coins[0]).await.unwrap();

        let auth_path = wallet.prepare_auth_path(cm_idx).await.unwrap();

        let old_coin = wallet
            .convert_to_old_coin(&coins[0], auth_path.clone())
            .unwrap();

        let sn = old_coin.compute_sn().unwrap().to_bytes();

        let merkle_rt = wallet
            .prepare_merkle_rt(&coins[0], auth_path.clone())
            .unwrap();

        (old_coin, sn, merkle_rt)
    };

    let (old_coin_2, sn_2, merkle_rt_2) = {
        let cm_idx = wallet.prepare_cm_idx(&coins[1]).await.unwrap();

        let auth_path = wallet.prepare_auth_path(cm_idx).await.unwrap();

        let old_coin = wallet
            .convert_to_old_coin(&coins[1], auth_path.clone())
            .unwrap();

        let sn = old_coin.compute_sn().unwrap().to_bytes();

        let merkle_rt = wallet
            .prepare_merkle_rt(&coins[1], auth_path.clone())
            .unwrap();

        (old_coin, sn, merkle_rt)
    };

    let (new_coin_1, new_coin_2) = wallet
        .prepare_2_new_coin_records(coins[0].v + coins[1].v)
        .unwrap();

    let pi = wallet
        .prepare_proof_2_to_2(
            old_coin_1,
            old_coin_2,
            new_coin_1.extract_new_coin(),
            new_coin_2.extract_new_coin(),
        )
        .unwrap();

    // open_ch contract execute call
    let args = {
        let (_, eph_pub_key) = SakKey::generate();

        let channel = Channel::new(
            // ch_id
            "ch_id".to_string(),
            // eph_key
            sak_crypto::encode_hex(
                &eph_pub_key.to_encoded_point(false).to_bytes(),
            ),
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

        serde_json::to_vec(&open_ch_params).unwrap()
    };

    let ctr_request = CtrRequest {
        req_type: request_type::OPEN_CH.to_string(),
        args,
        ctr_call_type: sak_contract_std::CtrCallType::Execute,
    };

    let json_response = saksaha::send_tx_pour(
        wallet.saksaha_endpoint.clone(),
        vec![sn_1, sn_2],
        vec![new_coin_1.cm.to_bytes(), new_coin_2.cm.to_bytes()],
        vec![merkle_rt_1, merkle_rt_2],
        pi,
        // ctr_addr,
        ENVELOPE_CTR_ADDR.to_string(),
        ctr_request,
    )
    .await
    .unwrap();

    println!("json: {:#?}", json_response);

    assert_eq!(json_response.error.is_none(), true)
}
