use super::utils;
use super::utils::TestContext;
use crate::tests::SaksahaTestUtils;
use sak_credential::CredentialProfile;
use sak_types::BlockCandidate;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_two_nodes_tx_pool_marshal_check_true() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test_1", "test_2"]);

    let test_credential_1 = CredentialProfile::test_1();
    let test_credential_2 = CredentialProfile::test_2();

    SaksahaTestUtils::init_test(&[
        &test_credential_1.public_key_str,
        &test_credential_2.public_key_str,
    ]);

    let test_context_1 = utils::make_test_context(
        // "test_1".to_string(),
        Some(35519),
        Some(35518),
        test_credential_1.secret,
        // String::from(
        //     "\
        //         7297b903877a957748b74068d63d6d566\
        //         148197524099fc1df5cd9e8814c66c7",
        // ),
        test_credential_1.public_key_str,
        // String::from(
        //     "\
        //         045739d074b8722891c307e8e75c9607e\
        //         0b55a80778b42ef5f4640d4949dbf3992\
        //         f6083b729baef9e9545c4e95590616fd3\
        //         82662a09653f2a966ff524989ae8c0f",
        // ),
        Some(true),
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_1,
        local_node: local_node_1,
        machine: machine_1,
        ..
    } = test_context_1;

    let test_context_2 = utils::make_test_context(
        // "test_2".to_string(),
        Some(35521),
        Some(35520),
        test_credential_2.secret,
        // String::from(
        //     "\
        //         aa99cfd91cc6f3b541d28f3e0707f9c7b\
        //         cf05cf495308294786ca450b501b5f2",
        // ),
        test_credential_2.public_key_str,
        // String::from(
        //     "\
        //         04240874d8c323c22a571f735e835ed2\
        //         f0619893a3989e557b1c9b4c699ac92b\
        //         84d0dc478108629c0353f2876941f90d\
        //         4b36346bcc19c6b625422adffb53b3a6af",
        // ),
        Some(false),
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_2,
        local_node: local_node_2,
        machine: machine_2,
        ..
    } = test_context_2;

    let dummy_tx1 = sak_types::mock_pour_tc_random();

    let dummy_tx2 = sak_types::mock_pour_tc_random();

    let block = BlockCandidate {
        validator_sig: String::from(""),
        tx_candidates: vec![dummy_tx1.clone(), dummy_tx2.clone()],
        witness_sigs: vec![],
        created_at: String::from(""),
    };

    {
        let machine_1 = machine_1.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run());
        });

        let machine_2 = machine_2.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run());
        });
    }

    machine_1
        .ledger
        .dist_ledger
        .apis
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    machine_1
        .ledger
        .dist_ledger
        .apis
        .send_tx(dummy_tx2.clone())
        .await
        .expect("Node should be able to send a transaction");

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        let tx_pool_2_contains_tx1 = machine_2
            .ledger
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        let tx_pool_2_contains_tx2 = machine_2
            .ledger
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx2.get_tx_hash())
            .await;

        assert!(tx_pool_2_contains_tx1);
        assert!(tx_pool_2_contains_tx2);
        println!("test1 passed");
    }

    {
        machine_1
            .ledger
            .dist_ledger
            .apis
            .write_block(Some(block))
            .await
            .expect("Block should be written");

        let tx_pool_1_contains_tx1 = machine_1
            .ledger
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        assert!(!tx_pool_1_contains_tx1);
    }
}
