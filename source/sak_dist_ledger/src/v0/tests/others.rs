use super::utils;
use crate::SyncPool;
use sak_contract_std::Storage;
use sak_types::{BlockCandidate, Tx, TxCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_insert_genesis_block_and_check_wrong_block_hash() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let dist_ledger = utils::make_dist_ledger().await;

    let gen_block = dist_ledger
        .get_block_by_height(&0)
        .await
        .unwrap()
        .expect("gen block should exist");

    let get_gen_hash = gen_block.get_block_hash();
    let gen_tx_hashes = &gen_block.tx_hashes;

    for tx_hash in gen_tx_hashes {
        let tx = match dist_ledger.get_tx(&tx_hash).await {
            Ok(t) => t,
            Err(err) => panic!("Error : {}", err),
        };

        let tx = tx.unwrap();

        assert_eq!(tx_hash, tx.get_tx_hash());
    }

    assert_ne!(get_gen_hash, &String::from("false hash"));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_set_and_get_contract_state_to_db() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let blockchain = utils::make_dist_ledger().await;
    let db = blockchain.ledger_db;

    let (contract_addr, ctr_state) = utils::make_dummy_state();

    db.batch_put_ctr_state(&contract_addr, &ctr_state)
        .await
        .expect("contract state should be saved");

    assert_eq!(
        db.get_ctr_state(&contract_addr)
            .expect("Contract State should be exist")
            .unwrap()
            .get(&contract_addr)
            .unwrap(),
        &ctr_state.clone()
    );
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn test_insert_invalid_contract_to_tx_pool() {
    let test_wasm = include_bytes!("./test_invalid_contract.wasm").to_vec();

    let dummy_tx = TxCandidate::new_dummy_pour_1_2_3();

    let sync_pool = SyncPool::new();

    sync_pool.insert_tx(dummy_tx).await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_a_genesis_block() {
    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_ctr_when_dist_ledger_writes_a_new_block() {
    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    dist_ledger
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_ctr_and_invoke_query_when_dist_ledger_writes_new_blocks() {
    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    println!("\n[+] Block1: Deploying test validator contract");
    dist_ledger
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");

    println!("\n[+] Block2: Query::get_validator");
    dist_ledger
        .write_block(utils::make_dummy_block_candidate_with_query_tx())
        .await
        .expect("Block_2 must be written");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_ctr_and_invoke_execute_and_query_when_dist_ledger_writes_new_blocks(
) {
    let ctr_addr: &String = &"test_wasm".to_string();

    let dist_ledger = utils::make_dist_ledger().await;

    dist_ledger.run().await;

    println!("\n[+] Block1: Deploying test validator contract");
    dist_ledger
        .write_block(utils::make_dummy_block_candidate_1())
        .await
        .expect("Block_1 must be written");

    println!("\n[+] Block2: Execute::add_validator");
    dist_ledger
        .write_block(utils::make_dummy_block_candidate_with_execute_tx())
        .await
        .expect("Block_2 must be written");

    println!("\n[+] Block3: Query::get_validator");
    dist_ledger
        .write_block(utils::make_dummy_block_candidate_with_query_tx())
        .await
        .expect("Block_3 must be written");

    {
        let result: Storage =
            dist_ledger.get_ctr_state(ctr_addr).await.unwrap().unwrap();

        println!("[*] result: {:#?}", result);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_repeating_write_block() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let blockchain = utils::make_dist_ledger().await;

    for i in 0..10000 as u64 {
        let block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![TxCandidate::new_dummy_pour_1_2_3()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            // block_height: i as u128,
            // merkle_root: String::from("2022061515340000"),
        };

        match blockchain.write_block(Some(block)).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        let tx_height =
            blockchain.get_latest_tx_height().await.unwrap().unwrap();

        println!("tx_height: {}", tx_height);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rpc_client_and_repeating_write_block_and_get_tx_height() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let blockchain = utils::make_dist_ledger().await;

    let repeat = 100;

    for i in 0..repeat as u64 {
        let block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![
                TxCandidate::new_dummy_pour_1_2_3(),
                TxCandidate::new_dummy_pour_2(),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            // block_height: i as u128,
            // merkle_root: String::from("2022061515340000"),
        };

        println!("eeeeeeeeeeeeeeeee");
        match blockchain.write_block(Some(block)).await {
            Ok(v) => v,
            Err(err) => panic!("Failed to write dummy block, err: {}", err),
        };

        let tx_height =
            blockchain.get_latest_tx_height().await.unwrap().unwrap();

        println!("tx_height: {}", tx_height);
    }

    let tx_height = blockchain.get_latest_tx_height().await.unwrap().unwrap();

    assert_eq!(2 * repeat - 1, tx_height);
}
