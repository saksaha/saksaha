use super::utils::DistLedgerTestUtils;
use crate::v0::testing;
use sak_kv_db::WriteBatch;
use sak_types::{BlockCandidate, Tx, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_put_and_get_transaction() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let bc = sak_types::mock_block_2();

    let block_hash = dist_ledger
        .write_block(Some(bc))
        .await
        .expect("block should be written")
        .unwrap();

    println!("[+] block hash: {:?}", block_hash);

    let tx_hashes = dist_ledger
        .ledger_db
        .get_tx_hashes(&block_hash)
        .expect("block should be written")
        .unwrap();

    for (idx, tx_hash) in tx_hashes.iter().enumerate() {
        let tx_val_retrieved = dist_ledger
            .get_tx(tx_hash)
            .await
            .expect("Tx should exist")
            .expect("tx should exist");

        assert_eq!(tx_val_retrieved.get_tx_hash(), &tx_hashes[idx]);

        println!(" tx_hash : {:?}", &tx_hashes[idx]);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_put_a_single_pour_tx() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let mut write_batch = WriteBatch::default();

    {
        let dummy_pour_tx = sak_types::mock_pour_tc_2to2_1().upgrade(2);

        let _dummy_tx_hash = dist_ledger
            .ledger_db
            .batch_put_tx(&mut write_batch, &dummy_pour_tx)
            .expect("pour_tx should be written");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_tx_mint_put_and_get_cm_idx() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let mint_tc = sak_types::mock_mint_tc_1();

    let mock_tx_hash = mint_tc.get_tx_hash().to_string();

    println!("mock_tx_hash :{:?}", mock_tx_hash);

    let block_candidate = BlockCandidate {
        validator_sig: "validator_sig".to_string(),
        tx_candidates: vec![mint_tc],
        witness_sigs: vec![],
        created_at: "created_at".to_string(),
    };

    dist_ledger
        .write_block(Some(block_candidate))
        .await
        .unwrap();

    let cms = dist_ledger
        .ledger_db
        .get_tx(&mock_tx_hash)
        .await
        .unwrap()
        .expect("cms should be obtained")
        .get_cms()
        .to_owned();

    let cm_1_idx = {
        println!("cms :{:?}", cms);

        let cm_1_idx = dist_ledger
            .ledger_db
            .get_cm_idx_by_cm(&cms[0])
            .unwrap()
            .expect("cm_1_idx should be obtained");

        cm_1_idx
    };

    println!("cm_1_idx : {:?}", cm_1_idx);
    assert_eq!(2, cm_1_idx);

    println!("[+] test pass");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_tx_pour_put_and_get_cm_idx() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let pour_tc = sak_types::mock_pour_tc_2to2_1();

    let mock_tx_hash = pour_tc.get_tx_hash().to_string();

    let block_candidate = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![pour_tc],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: format!("{}", 0),
    };

    dist_ledger
        .write_block(Some(block_candidate))
        .await
        .unwrap();

    let cms = dist_ledger
        .ledger_db
        .get_tx(&mock_tx_hash)
        .await
        .unwrap()
        .expect("tx should exist")
        .get_cms()
        .to_owned();

    let cm_1_idx = {
        let cm_1_idx = dist_ledger
            .ledger_db
            .get_cm_idx_by_cm(&cms[0])
            .unwrap()
            .expect("cm_1_idx should be obtained");

        cm_1_idx
    };

    let cm_2_idx = {
        let cm_2_idx = dist_ledger
            .ledger_db
            .get_cm_idx_by_cm(&cms[1])
            .expect("cm_2_idx should be obtained")
            .unwrap();
        cm_2_idx
    };

    println!("cm_1_idx : {:?}, cm_2_idx : {:?}", cm_1_idx, cm_2_idx);
    assert_eq!(2, cm_1_idx);
    assert_eq!(3, cm_2_idx);

    println!("[+] test pass");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_verify_proof_success() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let bc_1 = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![sak_types::mock_pour_tc_2to2_1()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: format!("{}", 0),
    };

    {
        let block_hash = dist_ledger
            .write_block(Some(bc_1))
            .await
            .expect("block should be written");

        println!("[+] dummy pour_tx hash: {:?}", block_hash);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_dist_ledger_verify_proof_fail() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let bc_1 = sak_types::mock_block_invalid_pour();

    {
        let block_hash = dist_ledger
            .write_block(Some(bc_1))
            .await
            .expect("block should be written");

        println!("[+] dummy pour_tx hash: {:?}", block_hash);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_double_spending_success() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![sak_types::mock_pour_tc_2to2_1()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: format!("{}", 0),
    };

    {
        let block_hash = dist_ledger
            .write_block(Some(block))
            .await
            .expect("block should be written");

        println!("[+] dummy pour_tx hash: {:?}", block_hash);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_double_spending_fail() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let bc_1 = sak_types::mock_block_pour_single();

    let bc_2 = sak_types::mock_block_pour_single();

    {
        let block_hash = dist_ledger
            .write_block(Some(bc_1))
            .await
            .expect("block should be written");

        println!("[+] dummy pour_tx hash: {:?}", block_hash);
    }

    {
        let block_hash = dist_ledger
            .write_block(Some(bc_2))
            .await
            .expect("block should be written");

        assert_eq!(None, block_hash);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_verify_merkle_rt_fail() {
    DistLedgerTestUtils::init_test(vec!["test"]);

    let dist_ledger = testing::mock_dist_ledger_1().await;

    let bc = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![sak_types::mock_pour_tc_random()],
        witness_sigs: vec![String::from("1")],
        created_at: format!("{}", 1),
    };

    let result = match dist_ledger.write_block(Some(bc)).await {
        Ok(v) => v,
        Err(err) => panic!("Failed to write dummy block, err: {}", err),
    };

    assert_eq!(None, result);
}
