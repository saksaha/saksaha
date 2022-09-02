use super::utils;

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

// #[tokio::test(flavor = "multi_thread")]
// async fn test_send_multiple_tx_pour() {
//     sak_test_utils::init_test_log();

//     // let test_credential = utils::make_test_credential().await;

//     let test_context = utils::mock_wallet_context().await;

//     let MockContext {
//         wallet,
//         rpc,
//         acc_addr,
//     } = test_context;

//     tokio::spawn(async move { rpc.run().await });

//     {
//         let balance = wallet.get_balance(&acc_addr).await.unwrap();
//         println!("[+] BALANCE {:?}", balance);

//         utils::send_msg_for_test(&acc_addr).await;

//         utils::mock_update_coin_status(&acc_addr).await;
//     }

//     {
//         let balance = wallet.get_balance(&acc_addr).await.unwrap();
//         println!("[+] BALANCE {:?}", balance);

//         utils::send_msg_for_test(&acc_addr).await;

//         utils::mock_update_coin_status(&acc_addr).await;
//     }

//     {
//         let balance = wallet.get_balance(&acc_addr).await.unwrap();
//         println!("[+] BALANCE {:?}", balance);

//         utils::send_msg_for_test(&acc_addr).await;

//         utils::mock_update_coin_status(&acc_addr).await;
//     }

//     {
//         let balance = wallet.get_balance(&acc_addr).await.unwrap();
//         println!("[+] BALANCE {:?}", balance);
//     }
// }
