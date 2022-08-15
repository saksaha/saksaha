use super::{test_util::TestUtil, utils};
use sak_types::{Block, BlockCandidate, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_genesis_block_and_check_wrong_block_hash() {
    sak_test_utils::init_test_log();

    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    let gen_block = dist_ledger
        .apis
        .get_block_by_height(&0)
        .await
        .unwrap()
        .expect("gen block should exist");

    let get_gen_hash = gen_block.get_block_hash();
    let gen_tx_hashes = &gen_block.tx_hashes;

    for tx_hash in gen_tx_hashes {
        let tx = match dist_ledger.apis.get_tx(&tx_hash).await {
            Ok(t) => t,
            Err(err) => panic!("Error : {}", err),
        };

        let tx = tx.unwrap();

        assert_eq!(tx_hash, tx.get_tx_hash());
    }

    assert_ne!(get_gen_hash, &String::from("false hash"));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_a_genesis_block() {
    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_a_new_block_after_genesis() {
    sak_test_utils::init_test_log();

    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    dist_ledger
        .apis
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_ctr_and_invoke_query_when_dist_ledger_writes_new_blocks() {
    sak_test_utils::init_test_log();

    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    println!("\n[+] Block1: Deploying test validator contract");
    dist_ledger
        .apis
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");

    println!("\n[+] Block2: Query::get_validator");
    dist_ledger
        .apis
        .write_block(utils::make_dummy_block_candidate_with_query_tx())
        .await
        .expect("Block_2 must be written");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequential_write_block() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    for i in 0..100 as u64 {
        let block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![TxCandidate::new_dummy_pour_m1_to_p3_p4()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: format!("{}", i),
        };

        match dist_ledger.apis.write_block(Some(block)).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequential_write_block_and_get_tx_height() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    let repeat = 100;

    for i in 0..repeat as u64 {
        let block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![
                TxCandidate::new_dummy_pour_m1_to_p3_p4(),
                TxCandidate::new_dummy_pour_2(),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: format!("{}", i),
        };

        match dist_ledger.apis.write_block(Some(block)).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };
    }

    let tx_height = dist_ledger
        .apis
        .get_latest_tx_height()
        .await
        .unwrap()
        .unwrap();

    assert_eq!(2 * repeat - 1 + 2, tx_height);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_block_and_check_merkle_rt_changed() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    for i in 0..5 as u64 {
        let cm: [u8; 32] = [i as u8; 32];

        let bc = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![TxCandidate::new_dummy_pour_variant_cm(cm)],
            witness_sigs: vec![String::from("1")],
            created_at: format!("{}", i),
        };

        match dist_ledger.apis.write_block(Some(bc)).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        let merkle_rt = dist_ledger
            .apis
            .get_latest_block_merkle_rt()
            .await
            .unwrap()
            .unwrap();

        println!("merkle_rt: {:?}", merkle_rt);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequential_sync_block_if_block_is_correct() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    let repeat = 5;

    for i in 1..repeat as u64 {
        let txs = utils::make_dummy_txs();

        let block = Block::new(
            String::from("validator_sig"),
            vec![String::from("tx_hashes")],
            vec![String::from("witness_sigs")],
            format!("{}", i),
            i as u128,
            [0; 32],
            i as u128,
        );

        match dist_ledger.apis.sync_block(block, txs).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };
    }

    let latest_block_height =
        dist_ledger.apis.get_latest_block_height().unwrap().unwrap();

    assert_eq!(latest_block_height, repeat - 1);
}

#[test]
fn deserialize_test() {
    let v = [
        91, 123, 34, 99, 104, 95, 105, 100, 34, 58, 34, 48, 52, 50, 97, 97, 55,
        98, 53, 97, 48, 99, 100, 53, 57, 52, 50, 50, 52, 51, 101, 49, 53, 97,
        97, 52, 99, 49, 54, 97, 48, 98, 55, 100, 102, 97, 57, 54, 57, 56, 101,
        48, 55, 100, 98, 97, 56, 98, 50, 99, 57, 50, 57, 102, 55, 53, 54, 51,
        102, 48, 100, 99, 49, 55, 53, 100, 53, 102, 57, 53, 54, 57, 51, 48, 48,
        48, 97, 50, 56, 51, 53, 57, 98, 54, 57, 57, 55, 57, 97, 101, 56, 97,
        51, 51, 98, 102, 50, 99, 50, 50, 55, 53, 49, 57, 55, 56, 54, 53, 102,
        53, 50, 56, 50, 49, 101, 101, 99, 57, 102, 102, 52, 52, 99, 101, 99,
        54, 55, 53, 101, 100, 101, 34, 44, 34, 101, 112, 104, 95, 107, 101,
        121, 34, 58, 34, 91, 52, 44, 50, 52, 53, 44, 50, 53, 51, 44, 49, 57,
        50, 44, 51, 44, 49, 52, 48, 44, 50, 50, 53, 44, 55, 44, 56, 55, 44, 57,
        51, 44, 52, 54, 44, 50, 51, 57, 44, 49, 53, 51, 44, 53, 54, 44, 50, 55,
        44, 49, 49, 49, 44, 49, 57, 50, 44, 49, 56, 50, 44, 49, 54, 57, 44, 49,
        49, 56, 44, 50, 56, 44, 49, 50, 54, 44, 50, 50, 55, 44, 49, 55, 52, 44,
        50, 52, 54, 44, 56, 53, 44, 53, 48, 44, 49, 48, 57, 44, 49, 54, 44, 57,
        56, 44, 49, 51, 49, 44, 49, 49, 44, 49, 48, 48, 44, 54, 53, 44, 53, 52,
        44, 49, 53, 57, 44, 50, 49, 48, 44, 52, 56, 44, 52, 44, 49, 55, 44, 49,
        50, 51, 44, 49, 48, 56, 44, 55, 53, 44, 49, 57, 54, 44, 49, 53, 51, 44,
        57, 51, 44, 53, 55, 44, 49, 55, 57, 44, 50, 51, 52, 44, 50, 49, 49, 44,
        49, 53, 53, 44, 51, 53, 44, 49, 57, 48, 44, 50, 50, 54, 44, 56, 51, 44,
        50, 49, 56, 44, 50, 50, 51, 44, 49, 54, 53, 44, 51, 50, 44, 49, 56, 52,
        44, 55, 55, 44, 53, 48, 44, 54, 49, 44, 50, 50, 52, 44, 50, 52, 48, 93,
        34, 44, 34, 115, 105, 103, 34, 58, 34, 91, 49, 54, 57, 44, 54, 48, 44,
        50, 52, 44, 50, 51, 48, 44, 49, 51, 44, 49, 51, 52, 44, 49, 52, 51, 44,
        56, 51, 44, 54, 48, 44, 49, 49, 53, 44, 50, 49, 53, 44, 56, 52, 44, 50,
        49, 56, 44, 49, 51, 54, 44, 55, 48, 44, 55, 52, 44, 49, 52, 53, 44, 50,
        50, 54, 44, 49, 52, 49, 44, 57, 53, 44, 49, 51, 48, 44, 50, 51, 49, 44,
        49, 51, 54, 44, 49, 55, 54, 44, 50, 51, 54, 44, 50, 52, 57, 44, 49, 53,
        48, 44, 56, 51, 44, 49, 50, 49, 44, 50, 51, 51, 44, 49, 54, 53, 44, 50,
        53, 48, 44, 49, 51, 48, 44, 49, 56, 51, 44, 49, 52, 56, 44, 50, 51, 53,
        44, 49, 54, 44, 51, 57, 44, 49, 51, 57, 44, 56, 51, 44, 49, 53, 53, 44,
        50, 49, 57, 44, 54, 53, 44, 49, 52, 55, 44, 49, 51, 56, 44, 49, 55, 52,
        44, 49, 53, 52, 44, 48, 44, 49, 50, 55, 44, 50, 50, 53, 44, 55, 50, 44,
        49, 53, 57, 44, 53, 49, 44, 49, 52, 57, 44, 49, 55, 48, 44, 49, 48, 54,
        44, 49, 56, 50, 44, 50, 48, 57, 44, 50, 44, 49, 56, 57, 44, 53, 56, 44,
        49, 54, 52, 44, 49, 49, 57, 44, 51, 44, 49, 52, 50, 44, 50, 51, 51, 44,
        56, 44, 57, 53, 44, 49, 56, 54, 44, 50, 53, 50, 44, 49, 51, 56, 44, 50,
        49, 54, 44, 49, 57, 55, 44, 51, 52, 44, 55, 56, 44, 49, 52, 49, 44, 53,
        57, 44, 55, 51, 44, 49, 50, 48, 44, 49, 53, 48, 44, 57, 53, 44, 49, 49,
        57, 44, 54, 44, 49, 53, 57, 44, 50, 52, 53, 44, 49, 53, 57, 44, 53, 49,
        44, 49, 51, 49, 44, 52, 50, 44, 50, 49, 48, 44, 52, 56, 44, 49, 57, 53,
        44, 49, 52, 49, 44, 53, 55, 44, 49, 54, 49, 44, 49, 56, 44, 54, 49, 44,
        50, 52, 49, 44, 50, 48, 52, 44, 49, 54, 53, 44, 49, 50, 48, 44, 56, 44,
        50, 51, 53, 44, 49, 52, 51, 44, 49, 50, 44, 49, 49, 44, 51, 50, 44, 54,
        51, 44, 50, 51, 54, 44, 54, 51, 44, 49, 49, 57, 44, 49, 56, 56, 44, 49,
        56, 49, 44, 49, 55, 53, 44, 49, 57, 56, 44, 49, 51, 51, 44, 49, 49, 52,
        44, 55, 49, 44, 49, 49, 52, 44, 49, 52, 57, 44, 49, 51, 52, 44, 56, 48,
        44, 49, 50, 56, 44, 49, 52, 56, 44, 49, 48, 54, 44, 50, 52, 54, 44, 49,
        49, 44, 50, 53, 51, 44, 49, 49, 54, 44, 49, 57, 55, 44, 54, 56, 44, 56,
        54, 44, 49, 54, 48, 93, 34, 125, 93,
    ];

    let serde_v: String = serde_json::from_slice(&v).unwrap();
    println!("{}", serde_v);
}
