use super::utils::{self, MockContext};

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

    let response = utils::mock_send_pour_tx(rpc_port, &acc_addr).await;

    println!("response: {:?}", response);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_2send_tx_twice() {
    sak_test_utils::init_test_log();

    // let test_credential = utils::make_test_credential().await;

    let test_context = utils::mock_wallet_context().await;

    let MockContext {
        wallet,
        rpc,
        acc_addr,
    } = test_context;

    tokio::spawn(async move { rpc.run().await });

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("[+] BALANCE {:?}", balance);

        utils::send_msg_for_test(&acc_addr).await;

        utils::update_coin_status(&acc_addr).await;
    }

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("[+] BALANCE {:?}", balance);

        utils::send_msg_for_test(&acc_addr).await;

        utils::update_coin_status(&acc_addr).await;
    }

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("[+] BALANCE {:?}", balance);

        utils::send_msg_for_test(&acc_addr).await;

        utils::update_coin_status(&acc_addr).await;
    }

    {
        let balance = wallet.get_balance(&acc_addr).await.unwrap();
        println!("[+] BALANCE {:?}", balance);
    }
}
