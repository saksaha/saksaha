use super::utils;

#[tokio::test(flavor = "multi_thread")]
async fn test_put_and_get_transaction() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let dist_ledger = utils::make_dist_ledger().await;

    let dummy_tx_values = utils::make_dummy_txs();

    let mut tx_hashes = vec![];

    for tx_val in dummy_tx_values.iter() {
        let h = dist_ledger
            .apis
            .ledger_db
            .schema
            .put_tx(&tx_val)
            .expect("Tx should be written");

        tx_hashes.push(h);
    }

    for (idx, tx_hash) in tx_hashes.iter().enumerate() {
        let tx_val_retrieved = dist_ledger
            .apis
            .get_tx(tx_hash)
            .await
            .expect("Tx should exist")
            .expect("tx should exist");

        assert_eq!(
            tx_val_retrieved.get_data(),
            dummy_tx_values[idx].get_data()
        );
    }
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_wrongful_put_and_get_transaction() {
//     sak_test_utils::init_test_log();
//     sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

//     let dist_ledger = utils::make_dist_ledger().await;
//     // let db = blockchain.ledger_db;

//     let dummy_tx_values = utils::make_dummy_txs();
//     let mut tx_hashes = vec![];

//     for tx_val in dummy_tx_values.iter() {
//         let h = dist_ledger
//             .apis
//             .put_tx(&tx_val)
//             .expect("Tx should be written");

//         tx_hashes.push(h);
//     }

//     let target_idx = 0;
//     let wrong_idx = 1;

//     let tx_val_retrieved = dist_ledger
//         .apis
//         .get_tx(&tx_hashes[target_idx])
//         .await
//         .expect("Tx should exist");

//     assert_ne!(
//         tx_val_retrieved.unwrap().get_data(),
//         dummy_tx_values[wrong_idx].get_data()
//     );
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn raw_iterator_to_first() {
//     sak_test_utils::init_test_log();
//     sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

//     let dist_ledger = utils::make_dist_ledger().await;
//     // let db = blockchain.ledger_db;

//     let dummy_tx_values = utils::make_dummy_txs();
//     let mut tx_hashes = vec![];

//     for tx_val in dummy_tx_values.iter() {
//         let h = db.put_tx(&tx_val).expect("Tx should be written");

//         tx_hashes.push(h);
//     }

//     let mut iter = db.kv_db.db_instance.raw_iterator_cf(
//         &db.kv_db.db_instance.cf_handle("created_at").unwrap(),
//     );

//     iter.seek_to_first();

//     let mut count = 0;
//     while iter.valid() {
//         println!(
//             "Saw {:?} {:?}",
//             std::str::from_utf8(iter.key().unwrap()),
//             std::str::from_utf8(iter.value().unwrap())
//         );
//         count = count + 1;
//         iter.next();
//     }
//     assert_eq!(count, tx_hashes.len());
// }
