use super::{test_util::TestUtil, utils};

#[tokio::test(flavor = "multi_thread")]
async fn test_put_and_get_transaction() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    let dummy_tx_values = utils::make_dummy_txs();

    let mut tx_hashes = vec![];

    let mut cm_idx_count = 0;

    for tx_val in dummy_tx_values.iter() {
        let h = dist_ledger
            .apis
            .ledger_db
            .schema
            .put_tx(&tx_val, &mut cm_idx_count)
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

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_put_a_single_pour_tx() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    {
        let dummy_pour_tx = utils::make_dummy_valid_pour_tx().await;

        let mut cm_idx_count = 0;

        let _dummy_tx_hash = dist_ledger
            .apis
            .ledger_db
            .schema
            .put_tx(&dummy_pour_tx, &mut cm_idx_count)
            .expect("pour_tx should be written");
    }

    println!("[+] test pass");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_dist_ledger_double_spending() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let dist_ledger = utils::make_dist_ledger().await;

    let mut cm_idx_count = 0;

    {
        let dummy_pour_tx = utils::make_dummy_valid_pour_tx().await;

        let dummy_tx_hash = dist_ledger
            .apis
            .ledger_db
            .schema
            .put_tx(&dummy_pour_tx, &mut cm_idx_count)
            .expect("pour_tx should be written");

        println!("[+] dummy pour_tx hash: {:?}", dummy_tx_hash);
    }

    {
        let dummy_pour_tx = utils::make_dummy_valid_pour_tx().await;

        let dummy_tx_hash = dist_ledger
            .apis
            .ledger_db
            .schema
            .put_tx(&dummy_pour_tx, &mut cm_idx_count)
            .expect("pour_tx should be written");

        println!("[+] dummy pour_tx hash: {:?}", dummy_tx_hash);
    }
}
