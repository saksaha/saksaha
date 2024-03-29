use crate::v0::testing;
use sak_types::{Block, BlockCandidate};

pub const REPEAT_NUM: u128 = 1;

// #[tokio::test(flavor = "multi_thread")]
// async fn test_insert_genesis_block_and_check_wrong_block_hash() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     let gen_block = dist_ledger
//         .get_block_by_height(&0)
//         .await
//         .unwrap()
//         .expect("gen block should exist");

//     let get_gen_hash = gen_block.get_block_hash();
//     let gen_tx_hashes = &gen_block.tx_hashes;

//     for tx_hash in gen_tx_hashes {
//         let tx = match dist_ledger.get_tx(&tx_hash).await {
//             Ok(t) => t,
//             Err(err) => panic!("Error : {}", err),
//         };

//         let tx = tx.unwrap();

//         assert_eq!(tx_hash, tx.get_tx_hash());
//     }

//     assert_ne!(get_gen_hash, &String::from("false hash"));
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_write_a_genesis_block() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     dist_ledger.run().await;
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_write_a_new_block_after_genesis() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     dist_ledger.run().await;

//     dist_ledger
//         .write_block(Some(sak_types::mock_block_2()))
//         .await
//         .expect("Block_1 must be written");
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sequential_write_block_1() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     for i in 0..REPEAT_NUM as u64 {
//         let block = BlockCandidate {
//             validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
//             tx_candidates: vec![sak_types::mock_pour_tc_2to2_1()],
//             witness_sigs: vec![String::from("1"), String::from("2")],
//             created_at: format!("{}", i),
//         };

//         match dist_ledger.write_block(Some(block)).await {
//             Ok(v) => v,
//             Err(err) => panic!("Failed to write dummy block, err: {}", err),
//         };
//     }
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sequential_write_block_and_get_tx_height() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     for i in 0..1 as u64 {
//         let block = BlockCandidate {
//             validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
//             tx_candidates: vec![sak_types::mock_pour_tc_2to2_1()],
//             witness_sigs: vec![String::from("1"), String::from("2")],
//             created_at: format!("{}", i),
//         };

//         match dist_ledger.write_block(Some(block)).await {
//             Ok(v) => v,
//             Err(err) => panic!("Failed to write dummy block, err: {}", err),
//         };
//     }

//     // TODO fix

//     // let tx_height = dist_ledger
//     //     .apis
//     //     .get_latest_tx_height()
//     //     .await
//     //     .unwrap()
//     //     .unwrap();

//     // assert_eq!(2 * repeat - 1 + 2, tx_height);
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_write_block_and_check_merkle_rt_changed() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     for i in 0..REPEAT_NUM as u64 {
//         let bc = BlockCandidate {
//             validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
//             tx_candidates: vec![sak_types::mock_pour_tc_2to2_1()],
//             witness_sigs: vec![String::from("1")],
//             created_at: format!("{}", i),
//         };

//         match dist_ledger.write_block(Some(bc)).await {
//             Ok(v) => v,
//             Err(err) => panic!("Failed to write dummy block, err: {}", err),
//         };

//         let merkle_rt = dist_ledger
//             .get_latest_block_merkle_rt()
//             .await
//             .unwrap()
//             .unwrap();

//         println!("merkle_rt: {:?}", merkle_rt);
//     }
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sequential_sync_block_if_block_is_correct() {
//     let dist_ledger = testing::mock_dist_ledger_1().await;

//     let latest_block_height = dist_ledger.get_latest_block_height().unwrap().unwrap();

//     println!("latest_block_height: {}", latest_block_height);

//     for i in 0..REPEAT_NUM as u64 {
//         // let txs = utils::make_dummy_txs();

//         let txs = vec![sak_types::mock_pour_tc_2to2_1().upgrade(2)];

//         let block = Block::new(
//             String::from("validator_sig"),
//             vec![String::from("tx_hashes")],
//             vec![String::from("witness_sigs")],
//             format!("{}", i),
//             i as u128,
//             [0; 32],
//             // i as u128,
//         );

//         let tx_candidates = txs.into_iter().map(|tx| tx.downgrade()).collect();

//         let bc_candidate = BlockCandidate {
//             validator_sig: block.validator_sig,
//             tx_candidates,
//             witness_sigs: block.witness_sigs,
//             created_at: block.created_at,
//         };

//         match dist_ledger.write_block(Some(bc_candidate)).await {
//             Ok(v) => v,
//             Err(err) => panic!("Failed to write dummy block, err: {}", err),
//         };
//     }

//     let latest_block_height = dist_ledger.get_latest_block_height().unwrap().unwrap();

//     assert_eq!(latest_block_height, REPEAT_NUM);
// }
