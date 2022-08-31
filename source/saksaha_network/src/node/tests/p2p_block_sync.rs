use super::utils::{make_test_context, TestContext};
use crate::tests::TestUtil;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_block_sync_true() {
    sak_test_utils::init_test_log();

    let app_prefix_vec = vec!["test_1", "test_2"];
    TestUtil::init_test(app_prefix_vec.clone());

    let test_context_1 = make_test_context(
        app_prefix_vec[0].to_string(),
        Some(35519),
        Some(35518),
        String::from(
            "\
                7297b903877a957748b74068d63d6d566\
                148197524099fc1df5cd9e8814c66c7",
        ),
        String::from(
            "\
                045739d074b8722891c307e8e75c9607e\
                0b55a80778b42ef5f4640d4949dbf3992\
                f6083b729baef9e9545c4e95590616fd3\
                82662a09653f2a966ff524989ae8c0f",
        ),
        true,
        // false,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_1,
        local_node: local_node_1,
        machine: machine_1,
        ..
    } = test_context_1;

    let test_context_2 = make_test_context(
        app_prefix_vec[1].to_string(),
        Some(35521),
        Some(35520),
        String::from(
            "\
                aa99cfd91cc6f3b541d28f3e0707f9c7b\
                cf05cf495308294786ca450b501b5f2",
        ),
        String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af",
        ),
        false,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_2,
        local_node: local_node_2,
        machine: machine_2,
        ..
    } = test_context_2;

    // let dummy_tx1 = sak_types::mock_pour_tc_m1_to_p3_p4();

    // let dummy_tx2 = sak_types::mock_pour_tc_2();

    let dummy_tx1 = sak_types::mock_pour_tc_1();

    let dummy_tx2 = sak_types::mock_pour_tc_2();

    tokio::time::sleep(Duration::from_secs(5)).await;

    {
        let machine_1 = machine_1.clone();
        let local_node_1 = local_node_1.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run());
        });

        let machine_2 = machine_2.clone();
        let local_node_2 = local_node_2.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run());
        });
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("Sending a tx1 to a node_1");

    machine_1
        .blockchain
        .dist_ledger
        .apis
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    tokio::time::sleep(Duration::from_secs(3)).await;

    {
        println!("check if node1 has tx1: {}", dummy_tx1.get_tx_hash());

        let tx_pool_1_contains_tx1 = machine_1
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        assert_eq!(tx_pool_1_contains_tx1, true, "node 1 should contain tx1");

        println!("[Success] node_1 has tx_1 (tx sent to node_1 directly)");

        println!("Checking if node2 has tx: {}", dummy_tx1.get_tx_hash());

        let tx_pool_2_contains_tx1 = machine_2
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        assert_eq!(
            tx_pool_2_contains_tx1, true,
            "tx pool 2 should contain tx 1"
        );

        println!("[Success] node_2 has tx_1 (shared from node_1)");
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(None)
            .await
            .expect("Block should be written");

        let last_height_1 = local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()
            .unwrap()
            .unwrap();

        assert_eq!(1, last_height_1);

        println!("test 2 passed");

        tokio::time::sleep(Duration::from_secs(2)).await;

        let last_height_2 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()
            .unwrap()
            .unwrap();

        assert_eq!(
            last_height_1, last_height_2,
            "two nodes have the same latest block height"
        );

        println!("test 3 passed");
    }

    tokio::time::sleep(Duration::from_secs(2)).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_late_block_sync_true() {
    sak_test_utils::init_test_log();

    let app_prefix_vec = vec!["test_1", "test_2"];
    TestUtil::init_test(app_prefix_vec.clone());

    let test_context_1 = make_test_context(
        app_prefix_vec[0].to_string(),
        Some(35519),
        Some(35518),
        String::from(
            "\
                7297b903877a957748b74068d63d6d566\
                148197524099fc1df5cd9e8814c66c7",
        ),
        String::from(
            "\
                045739d074b8722891c307e8e75c9607e\
                0b55a80778b42ef5f4640d4949dbf3992\
                f6083b729baef9e9545c4e95590616fd3\
                82662a09653f2a966ff524989ae8c0f",
        ),
        true,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_1,
        local_node: local_node_1,
        machine: machine_1,
        ..
    } = test_context_1;

    let test_context_2 = make_test_context(
        app_prefix_vec[1].to_string(),
        Some(35521),
        Some(35520),
        String::from(
            "\
                aa99cfd91cc6f3b541d28f3e0707f9c7b\
                cf05cf495308294786ca450b501b5f2",
        ),
        String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af",
        ),
        false,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_2,
        local_node: local_node_2,
        machine: machine_2,
        ..
    } = test_context_2;

    // let dummy_tx1 = sak_types::mock_pour_tc_m1_to_p3_p4();
    let dummy_tx1 = sak_types::mock_pour_tc_1();
    let dummy_tx2 = sak_types::mock_pour_tc_2();
    // let dummy_tx3 = sak_types::mock_pour_tc_3();
    let dummy_tx3 = sak_types::mock_pour_tc_2();

    tokio::time::sleep(Duration::from_secs(5)).await;

    {
        let machine_1 = machine_1.clone();
        let local_node_1 = local_node_1.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run());
        });
    }

    {
        tokio::time::sleep(Duration::from_secs(2)).await;

        println!("Sending a tx1 to a node_1 at a first time");

        machine_1
            .blockchain
            .dist_ledger
            .apis
            .send_tx(dummy_tx1.clone())
            .await
            .expect("Node should be able to send a transaction");

        tokio::time::sleep(Duration::from_secs(2)).await;

        local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(None)
            .await
            .expect("Block should be written");

        let last_height_1 = local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()
            .unwrap()
            .unwrap();

        assert_eq!(1, last_height_1);

        println!("last height is confirmed on node 1");
    }

    {
        tokio::time::sleep(Duration::from_secs(2)).await;

        println!("Sending a tx1 to a node_1 at a second time");

        machine_1
            .blockchain
            .dist_ledger
            .apis
            .send_tx(dummy_tx2.clone())
            .await
            .expect("Node should be able to send a transaction");

        tokio::time::sleep(Duration::from_secs(2)).await;

        local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(None)
            .await
            .expect("Block should be written");

        let last_height_1 = local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()
            .unwrap()
            .unwrap();

        assert_eq!(2, last_height_1);

        println!("last height is confirmed on node 2");
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        tokio::time::sleep(Duration::from_secs(2)).await;

        println!("Sending a tx1 to a node_1 at a third time");

        machine_1
            .blockchain
            .dist_ledger
            .apis
            .send_tx(dummy_tx3.clone())
            .await
            .expect("Node should be able to send a transaction");

        tokio::time::sleep(Duration::from_secs(2)).await;

        local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(None)
            .await
            .expect("Block should be written");

        let last_height_1 = local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()
            .unwrap()
            .unwrap();

        assert_eq!(3, last_height_1);

        println!("last height is confirmed on 3");
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        let machine_2 = machine_2.clone();
        let local_node_2 = local_node_2.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run());
        });
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    let last_height_2 = local_node_2
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_latest_block_height()
        .unwrap()
        .unwrap();

    assert_eq!(3, last_height_2);

    println!("last height of local_node_2 is confirmed on 3");
}
