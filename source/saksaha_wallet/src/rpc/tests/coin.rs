use super::utils;
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
