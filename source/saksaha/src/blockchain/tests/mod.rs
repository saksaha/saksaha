use super::*;

#[cfg(test)]
mod test {
    use super::blockchain::TxValue;
    use crate::blockchain::{
        ledger::ledger_columns, Blockchain, BlockchainArgs,
    };
    use file_system::FS;
    use hex;
    use rocksdb::WriteBatch;
    use sha3::{Digest, Sha3_256};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_blockchain() -> Blockchain {
        // let db_path = {
        //     let app_path = FS::create_or_get_app_path().unwrap();
        //     let db_path = app_path.join("db_ledger_test");
        //     let db_path = db_path.as_os_str().to_str().unwrap().to_owned();
        //     Some(db_path)
        // };

        let blockchain_args = BlockchainArgs {
            db_prefix: Some("test".to_string()),
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_values() -> Vec<(String, TxValue)> {
        let tx = TxValue {
            pi: "0x123",
            sig_vec: "0x0000",
            created_at: "1346546123",
            data: "None",
        };

        let transaction_hash_seed = vec!["foo", "bob", "sandy", "land", "god"];

        let mut values = vec![];

        for (_idx, tx_data) in transaction_hash_seed.iter().enumerate() {
            let tx_hash = {
                let mut h = Sha3_256::new();
                h.update(tx_data);
                h.finalize()
            };

            let tx_hash_str = hex::encode(tx_hash);

            let tx_value = TxValue {
                pi: tx.pi,
                sig_vec: tx.sig_vec,
                created_at: tx.created_at,
                data: tx.data,
            };

            values.push((tx_hash_str, tx_value))
        }

        values
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_and_get_transaction() {
        init();

        let blockchain = make_blockchain().await;

        let db = blockchain.ledger.ledger_db.db;

        let dummy_tx_values = make_dummy_values();

        dummy_tx_values.iter().for_each(|(tx_hash, tx_val)| {
            db.put_cf(
                db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                tx_hash,
                tx_val.created_at,
            )
            .expect("Tx crated_at should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::DATA).unwrap(),
                tx_hash,
                tx_val.data,
            )
            .expect("Tx data should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::PI).unwrap(),
                tx_hash,
                tx_val.pi,
            )
            .expect("Tx pi should be inserted");

            db.put_cf(
                db.cf_handle(ledger_columns::SIG_VEC).unwrap(),
                tx_hash,
                tx_val.sig_vec,
            )
            .expect("Tx sig_vec should be inserted");
        });

        dummy_tx_values.iter().for_each(|(tx_hash, _)| {
            let created_at = db
                .get_cf(
                    db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                    tx_hash,
                )
                .expect("created_at should be returned")
                .expect("created_at should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::CREATED_AT,
                std::str::from_utf8(&created_at)
            );

            let data = db
                .get_cf(db.cf_handle(ledger_columns::DATA).unwrap(), tx_hash)
                .expect("data should be returned")
                .expect("data should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::DATA,
                std::str::from_utf8(&data)
            );

            let sig_vec = db
                .get_cf(db.cf_handle(ledger_columns::SIG_VEC).unwrap(), tx_hash)
                .expect("sig_vec should be returned")
                .expect("sig_vec should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::SIG_VEC,
                std::str::from_utf8(&sig_vec)
            );

            let pi = db
                .get_cf(db.cf_handle(ledger_columns::PI).unwrap(), tx_hash)
                .expect("pi should be returned")
                .expect("pi should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::PI,
                std::str::from_utf8(&pi)
            );
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn batchput_and_get_transaction() {
        init();

        let blockchain = make_blockchain().await;

        let db = blockchain.ledger.ledger_db.db;

        let dummy_tx_values = make_dummy_values();

        let mut batch = WriteBatch::default();

        dummy_tx_values.iter().for_each(|(tx_hash, tx_val)| {
            batch.put_cf(
                db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                tx_hash,
                tx_val.created_at,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::DATA).unwrap(),
                tx_hash,
                tx_val.data,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::PI).unwrap(),
                tx_hash,
                tx_val.pi,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::SIG_VEC).unwrap(),
                tx_hash,
                tx_val.sig_vec,
            );
        });
        db.write(batch).expect("failed to batchWrite");

        dummy_tx_values.iter().for_each(|(tx_hash, _)| {
            let created_at = db
                .get_cf(
                    db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                    tx_hash,
                )
                .expect("created_at should be returned")
                .expect("created_at should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::CREATED_AT,
                std::str::from_utf8(&created_at)
            );

            let data = db
                .get_cf(db.cf_handle(ledger_columns::DATA).unwrap(), tx_hash)
                .expect("data should be returned")
                .expect("data should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::DATA,
                std::str::from_utf8(&data)
            );

            let sig_vec = db
                .get_cf(db.cf_handle(ledger_columns::SIG_VEC).unwrap(), tx_hash)
                .expect("sig_vec should be returned")
                .expect("sig_vec should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::SIG_VEC,
                std::str::from_utf8(&sig_vec)
            );

            let pi = db
                .get_cf(db.cf_handle(ledger_columns::PI).unwrap(), tx_hash)
                .expect("pi should be returned")
                .expect("pi should exist");

            println!(
                "key: {}, cf: {}, got value: {:?}",
                tx_hash,
                ledger_columns::PI,
                std::str::from_utf8(&pi)
            );
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn raw_iterator_to_first() {
        init();

        let blockchain = make_blockchain().await;

        let db = blockchain.ledger.ledger_db.db;

        let dummy_tx_values = make_dummy_values();

        let mut batch = WriteBatch::default();

        dummy_tx_values.iter().for_each(|(tx_hash, tx_val)| {
            batch.put_cf(
                db.cf_handle(ledger_columns::CREATED_AT).unwrap(),
                tx_hash,
                tx_val.created_at,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::DATA).unwrap(),
                tx_hash,
                tx_val.data,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::PI).unwrap(),
                tx_hash,
                tx_val.pi,
            );

            batch.put_cf(
                db.cf_handle(ledger_columns::SIG_VEC).unwrap(),
                tx_hash,
                tx_val.sig_vec,
            );
        });

        db.write(batch).expect("failed to batchWrite");

        let mut iter = db
            .raw_iterator_cf(db.cf_handle(ledger_columns::CREATED_AT).unwrap());

        iter.seek_to_first();

        while iter.valid() {
            println!(
                "Saw {:?} {:?}",
                std::str::from_utf8(iter.key().unwrap()),
                std::str::from_utf8(iter.value().unwrap())
            );
            iter.next();
        }
    }
}
