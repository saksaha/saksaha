use super::utils::make_dual_node_test_context;
use super::utils::DualNodeTestContext;
use crate::tests::SaksahaTestUtils;
use sak_credential::CredentialProfile;
use sak_dist_ledger::DistLedgerEvent;
use sak_logger::{error, info};
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_tx_sync_true() {
    // sak_test_utils::init_test_log();

    // let app_prefix_vec = vec!["test_1", "test_2"];
    // TestUtil::init_test(app_prefix_vec.clone());

    let test_credential_1 = CredentialProfile::test_1();
    let test_credential_2 = CredentialProfile::test_2();

    SaksahaTestUtils::init_test(&[
        &test_credential_1.public_key_str,
        &test_credential_2.public_key_str,
    ]);

    let DualNodeTestContext {
        p2p_host_1,
        local_node_1,
        machine_1,
        p2p_host_2,
        local_node_2,
        machine_2,
    } = make_dual_node_test_context(Some(false), Some(false)).await;

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

    let dummy_tx1 = sak_types::mock_pour_tc_random();
    println!(
        "Sending a tx1 to a node_1, tx_hash: {}",
        dummy_tx1.get_tx_hash()
    );

    machine_1
        .ledger
        .dist_ledger
        .apis
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    let tx_pool_1_contains_tx1 = machine_1
        .ledger
        .dist_ledger
        .apis
        .tx_pool_contains(dummy_tx1.get_tx_hash())
        .await;

    assert_eq!(tx_pool_1_contains_tx1, true);

    info!("[Success] node_1 has tx_1 (tx sent to node_1 directly)");

    let mut ledger_event_rx = machine_2.ledger.dist_ledger.ledger_event_tx.subscribe();

    let ev = tokio::time::timeout(Duration::from_secs(5), ledger_event_rx.recv())
        .await
        .unwrap()
        .unwrap();

    println!("ev: {:?}", ev);

    match ev {
        DistLedgerEvent::TxPoolStat(v) => {
            info!(
                "[Success] node_2 has tx_1 (shared from node_1), tx: {:?}",
                v
            );
        }
        _ => {
            error!("[panic] event: {:?}", ev);
            panic!()
        }
    }
}
