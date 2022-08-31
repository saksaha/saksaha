use sak_dist_ledger::DistLedgerEvent;
use sak_types::TxCandidate;
use type_extension::U8Array;

use super::utils::{get_two_dummy_nodes, make_test_context, TestContext};
use crate::tests::TestUtil;
use std::time::{Duration, Instant};

#[tokio::test(flavor = "multi_thread")]
async fn test_tx_sync_true() {
    sak_test_utils::init_test_log();

    let app_prefix_vec = vec!["test_1", "test_2"];
    TestUtil::init_test(app_prefix_vec.clone());

    let (
        p2p_host_1,
        local_node_1,
        machine_1,
        //
        p2p_host_2,
        local_node_2,
        machine_2,
    ) = get_two_dummy_nodes().await;

    let dummy_tx1 = sak_types::mock_pour_tc_1();

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

    println!("Sending a tx1 to a node_1");

    machine_1
        .blockchain
        .dist_ledger
        .apis
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    {
        println!("check if node1 has tx1: {}", dummy_tx1.get_tx_hash());

        let tx_pool_1_contains_tx1 = machine_1
            .blockchain
            .dist_ledger
            .apis
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        assert_eq!(tx_pool_1_contains_tx1, true);

        println!("[Success] node_1 has tx_1 (tx sent to node_1 directly)");

        println!("Checking if node2 has tx: {}", dummy_tx1.get_tx_hash());

        let mut ledger_event_rx = {
            let rx = machine_2
                .blockchain
                .dist_ledger
                .ledger_event_tx
                .read()
                .await
                .subscribe();

            rx
        };

        let mut is_pass: bool = false;

        let now = Instant::now();

        while now.elapsed() < Duration::from_millis(5000) {
            let ev = match ledger_event_rx.recv().await {
                Ok(e) => e,
                Err(err) => {
                    log::error!("Error receiving ledger event, err: {}", err);

                    continue;
                }
            };

            let _event_handle_res = match ev {
                DistLedgerEvent::TxPoolStat(_new_tx_hashes) => {
                    log::info!("test done");
                    is_pass = true;
                }
                _ => {}
            };
        }

        assert_eq!(is_pass, true);

        println!("[Success] node_2 has tx_1 (shared from node_1)");
    }

    // println!("[!] sleep 500 seconds..");
    // tokio::time::sleep(Duration::from_secs(500)).await;
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_tx_sync_broadcast_multiple_txs() {
//     sak_test_utils::init_test_log();

//     let app_prefix_vec = vec!["test_1", "test_2"];
//     TestUtil::init_test(app_prefix_vec.clone());

//     let test_context_1 = make_test_context(
//         app_prefix_vec[0].to_string(),
//         Some(35519),
//         Some(35518),
//         String::from(
//             "\
//                 7297b903877a957748b74068d63d6d566\
//                 148197524099fc1df5cd9e8814c66c7",
//         ),
//         String::from(
//             "\
//                 045739d074b8722891c307e8e75c9607e\
//                 0b55a80778b42ef5f4640d4949dbf3992\
//                 f6083b729baef9e9545c4e95590616fd3\
//                 82662a09653f2a966ff524989ae8c0f",
//         ),
//         true,
//         // false,
//     )
//     .await;

//     let TestContext {
//         p2p_host: p2p_host_1,
//         local_node: local_node_1,
//         machine: machine_1,
//         ..
//     } = test_context_1;

//     let test_context_2 = make_test_context(
//         app_prefix_vec[1].to_string(),
//         Some(35521),
//         Some(35520),
//         String::from(
//             "\
//                 aa99cfd91cc6f3b541d28f3e0707f9c7b\
//                 cf05cf495308294786ca450b501b5f2",
//         ),
//         String::from(
//             "\
//                 04240874d8c323c22a571f735e835ed2\
//                 f0619893a3989e557b1c9b4c699ac92b\
//                 84d0dc478108629c0353f2876941f90d\
//                 4b36346bcc19c6b625422adffb53b3a6af",
//         ),
//         false,
//     )
//     .await;

//     let TestContext {
//         p2p_host: p2p_host_2,
//         local_node: local_node_2,
//         machine: machine_2,
//         ..
//     } = test_context_2;

//     // let dummy_tx1 = sak_types::mock_pour_tc_m1_to_p3_p4();
//     // let dummy_tx2 = sak_types::mock_pour_tc_2();
//     let dummy_txs: Vec<TxCandidate> = vec![
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//         mock_pour_tc_variant_created_at(),
//     ];

//     {
//         let machine_1 = machine_1.clone();

//         let local_node_1 = local_node_1.clone();

//         tokio::spawn(async move {
//             tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run());
//         });

//         let machine_2 = machine_2.clone();

//         let local_node_2 = local_node_2.clone();

//         tokio::spawn(async move {
//             tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run());
//         });
//     }

//     println!("[!] sleep 10 seconds..");
//     tokio::time::sleep(Duration::from_secs(10)).await;

//     // clear the display
//     std::process::Command::new("clear").status().unwrap();

//     println!("Sending dummy_txs to node_1");

//     let _ = dummy_txs
//         .iter()
//         .for_each(|d| println!("dummy_tx hash: {:?}", d.get_tx_hash()));

//     println!("[!] sleep 10 seconds..");
//     tokio::time::sleep(Duration::from_secs(10)).await;

//     for idx in 0..dummy_txs.len() {
//         machine_1
//             .blockchain
//             .dist_ledger
//             .apis
//             .send_tx(dummy_txs[idx].clone())
//             .await
//             .expect("Node should be able to send a transaction");
//     }

//     println!("[!] sleep 10 seconds for waiting Tx Sync.., node_1 --> node_2");
//     tokio::time::sleep(Duration::from_secs(10)).await;
// }
