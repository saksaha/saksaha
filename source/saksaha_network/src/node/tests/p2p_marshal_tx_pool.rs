use super::utils;
use crate::p2p::P2PHost;
use crate::{machine::Machine, node::LocalNode};
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_types::{BlockCandidate, TxCandidate};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_two_nodes_tx_pool_marshal_check_true() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![
        String::from("test_1"),
        String::from("test_2"),
    ])
    .unwrap();

    let (p2p_host_1, local_node_1, machine_1, _, _): (
        P2PHost,
        Arc<LocalNode>,
        Arc<Machine>,
        Arc<PeerTable>,
        Arc<Identity>,
    ) = utils::create_client(
        "test_1".to_string(),
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
        // true,
    )
    .await;

    let (p2p_host_2, local_node_2, machine_2, _, _): (
        P2PHost,
        Arc<LocalNode>,
        Arc<Machine>,
        Arc<PeerTable>,
        Arc<Identity>,
    ) = utils::create_client(
        "test_2".to_string(),
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
        // false,
    )
    .await;

    let dummy_tx1 = TxCandidate::new_dummy_pour_m1_to_p3_p4();
    let dummy_tx2 = TxCandidate::new_dummy_pour_2();

    let block = {
        let c = BlockCandidate {
            validator_sig: String::from(""),
            tx_candidates: vec![dummy_tx1.clone(), dummy_tx2.clone()],
            witness_sigs: vec![],
            created_at: String::from(""),
        };

        c
    };

    {
        let local_node_1 = local_node_1.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run(),);
        });

        let local_node_2 = local_node_2.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run(),);
        });
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    local_node_1
        .machine
        .blockchain
        .dist_ledger
        .apis
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    local_node_1
        .machine
        .blockchain
        .dist_ledger
        .apis
        .send_tx(dummy_tx2.clone())
        .await
        .expect("Node should be able to send a transaction");

    tokio::time::sleep(Duration::from_secs(3)).await;

    {
        let tx_pool_2_contains_tx1 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        let tx_pool_2_contains_tx2 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx2.get_tx_hash())
            .await;

        assert_eq!(tx_pool_2_contains_tx1, true);
        assert_eq!(tx_pool_2_contains_tx2, true);
        println!("test1 passed");
    }

    {
        local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .write_block(Some(block))
            .await
            .expect("Block should be written");

        let tx_pool_1_contains_tx1 = local_node_1
            .machine
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        assert_eq!(tx_pool_1_contains_tx1, false);
    }
}
