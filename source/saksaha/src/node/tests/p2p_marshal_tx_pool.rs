use sak_types::{BlockCandidate, TxCandidate};
use std::time::Duration;

use super::utils::create_client;

#[tokio::test(flavor = "multi_thread")]
async fn test_two_nodes_tx_pool_marshal_check_true() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![
        String::from("test_1"),
        String::from("test_2"),
    ])
    .unwrap();

    let (p2p_host_1, local_node_1, machine_1) = create_client(
        "test_1".to_string(),
        Some(35519),
        Some(35518), // disc_port
        String::from(
            "4649b25129b6206cb9bedd7356ba17d57a0ff1e\
                    1939f02e01cf59ab2a61633bb",
        ),
        String::from(
            "\
                04fbd9336fcbb603a5cf80435e193c107eaf80cd3a7e93009f15\
                6c410444d59db3b3bcccae6bc6f736b43ee9542ee657955b94984\
                7dcedc79dd295950af9e87f03",
        ),
    )
    .await;

    let (p2p_host_2, local_node_2, machine_2) = create_client(
        "test_2".to_string(),
        Some(35521),
        Some(35520), // disc_port
        String::from(
            "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786\
                    ca450b501b5f2",
        ),
        String::from(
            "\
                    04240874d8c323c22a571f735e835ed2\
                    f0619893a3989e557b1c9b4c699ac92b\
                    84d0dc478108629c0353f2876941f90d\
                    4b36346bcc19c6b625422adffb53b3a6af",
        ),
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

    tokio::time::sleep(Duration::from_secs(2)).await;

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

    tokio::time::sleep(Duration::from_secs(2)).await;

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
