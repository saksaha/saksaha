use super::*;

#[cfg(test)]
mod test {
    use crate::{Blockchain, BlockchainArgs};
    use sak_types::Transaction;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_blockchain() -> Blockchain {
        let blockchain_args = BlockchainArgs {
            app_prefix: String::from("test"),
            tx_pool_sync_interval: None,
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_values() -> Vec<Transaction> {
        vec![
            Transaction {
                pi: String::from("0x111"),
                signature: String::from("0x1111"),
                created_at: String::from("1346546123"),
                data: String::from("one").as_bytes().to_vec(),
                contract: String::from("one").as_bytes().to_vec(),
            },
            Transaction {
                pi: String::from("0x222"),
                signature: String::from("0x2222"),
                created_at: String::from("1346546124"),
                data: String::from("two").as_bytes().to_vec(),
                contract: String::from("two").as_bytes().to_vec(),
            },
            Transaction {
                pi: String::from("0x333"),
                signature: String::from("0x3333"),
                created_at: String::from("1346546125"),
                data: String::from("three").as_bytes().to_vec(),
                contract: String::from("three").as_bytes().to_vec(),
            },
            Transaction {
                pi: String::from("0x444"),
                signature: String::from("0x4444"),
                created_at: String::from("1346546126"),
                data: String::from("four").as_bytes().to_vec(),
                contract: String::from("four").as_bytes().to_vec(),
            },
        ]
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_and_get_transaction() {
        init();

        let blockchain = make_blockchain().await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_values();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = tx_db
                .write_tx(tx_val.clone())
                .await
                .expect("Tx should be written");

            tx_hashes.push(h);
        }

        for (idx, tx_hash) in tx_hashes.iter().enumerate() {
            let tx_val_retrieved =
                tx_db.read_tx(&tx_hash).await.expect("Tx should exist");

            assert_eq!(tx_val_retrieved.data, dummy_tx_values[idx].data);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wrongful_put_and_get_transaction() {
        init();

        let blockchain = make_blockchain().await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_values();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = tx_db
                .write_tx(tx_val.clone())
                .await
                .expect("Tx should be written");

            tx_hashes.push(h);
        }

        let target_idx = 0;
        let wrong_idx = 1;

        let tx_val_retrieved = tx_db
            .read_tx(&tx_hashes[target_idx])
            .await
            .expect("Tx should exist");

        assert_ne!(tx_val_retrieved.data, dummy_tx_values[wrong_idx].data);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn raw_iterator_to_first() {
        init();

        let blockchain = make_blockchain().await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_values();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = tx_db
                .write_tx(tx_val.clone())
                .await
                .expect("Tx should be written");

            tx_hashes.push(h);
        }

        let mut iter = tx_db.iter();
        iter.seek_to_first();

        let mut count = 0;
        while iter.valid() {
            println!(
                "Saw {:?} {:?}",
                std::str::from_utf8(iter.key().unwrap()),
                std::str::from_utf8(iter.value().unwrap())
            );
            count = count + 1;
            iter.next();
        }
        assert_eq!(count, tx_hashes.len());
    }
}
