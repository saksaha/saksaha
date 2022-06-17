#[cfg(test)]
mod test {
    use crate::{Blockchain, BlockchainArgs};
    use sak_types::BlockCandidate;
    use sak_types::Hashable;
    use sak_types::Transaction;

    fn init() {
        let _ = env_logger::builder().is_test(true).init();
    }

    fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            transactions: vec![
                Transaction::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    String::from("1"),
                    vec![11, 11, 11],
                ),
                Transaction::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    String::from("2"),
                    vec![22, 22, 22],
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: String::from("0"),
        };

        genesis_block
    }

    async fn make_blockchain(gen_block: BlockCandidate) -> Blockchain {
        println!("(ToBeDeleted) Genesis Block : {:?}", gen_block);

        let blockchain_args = BlockchainArgs {
            app_prefix: String::from("test"),
            tx_pool_sync_interval: None,
            genesis_block: gen_block,
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_txs() -> Vec<Transaction> {
        vec![
            Transaction::new(
                String::from("1346546123"),
                String::from("one").as_bytes().to_vec(),
                String::from("0x111"),
                String::from("0x1111"),
                String::from("one").as_bytes().to_vec(),
            ),
            Transaction::new(
                String::from("1346546124"),
                String::from("two").as_bytes().to_vec(),
                String::from("0x222"),
                String::from("0x2222"),
                String::from("two").as_bytes().to_vec(),
            ),
            Transaction::new(
                String::from("1346546125"),
                String::from("three").as_bytes().to_vec(),
                String::from("0x333"),
                String::from("0x3333"),
                String::from("three").as_bytes().to_vec(),
            ),
            Transaction::new(
                String::from("1346546126"),
                String::from("four").as_bytes().to_vec(),
                String::from("0x444"),
                String::from("0x4444"),
                String::from("four").as_bytes().to_vec(),
            ),
        ]
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_and_get_transaction() {
        init();

        let gen_block = make_dummy_genesis_block();
        let blockchain = make_blockchain(gen_block).await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h =
                tx_db.write_tx(&tx_val).await.expect("Tx should be written");

            tx_hashes.push(h);
        }

        for (idx, tx_hash) in tx_hashes.iter().enumerate() {
            let tx_val_retrieved =
                tx_db.read_tx(&tx_hash).await.expect("Tx should exist");

            assert_eq!(
                tx_val_retrieved.get_data(),
                dummy_tx_values[idx].get_data()
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wrongful_put_and_get_transaction() {
        init();

        let gen_block = make_dummy_genesis_block();
        let blockchain = make_blockchain(gen_block).await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h =
                tx_db.write_tx(&tx_val).await.expect("Tx should be written");

            tx_hashes.push(h);
        }

        let target_idx = 0;
        let wrong_idx = 1;

        let tx_val_retrieved = tx_db
            .read_tx(&tx_hashes[target_idx])
            .await
            .expect("Tx should exist");

        assert_ne!(
            tx_val_retrieved.get_data(),
            dummy_tx_values[wrong_idx].get_data()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn raw_iterator_to_first() {
        init();

        let gen_block = make_dummy_genesis_block();
        let blockchain = make_blockchain(gen_block).await;
        let tx_db = blockchain.database.tx_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h =
                tx_db.write_tx(&tx_val).await.expect("Tx should be written");

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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_insert_genesis_block_and_check_tx() {
        init();

        let gen_block = make_dummy_genesis_block();
        let (block, _) = gen_block.extract();
        let gen_block_hash = block.get_hash();

        let blockchain = make_blockchain(gen_block).await;

        let gen_block = match blockchain.get_block(gen_block_hash).await {
            Ok(b) => b,
            Err(err) => panic!("Error : {}", err),
        };

        let get_gen_hash = gen_block.get_hash();

        let gen_tx_hashes = gen_block.get_tx_hashes();

        assert_eq!(get_gen_hash, gen_block_hash);

        for tx_hash in gen_tx_hashes {
            let tx = match blockchain.get_transaction(tx_hash).await {
                Ok(t) => t,
                Err(err) => panic!("Error : {}", err),
            };

            assert_eq!(tx_hash, tx.get_hash());
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_insert_genesis_block_and_check_wrong_block_hash() {
        init();

        let gen_block = make_dummy_genesis_block();
        let (block, _) = gen_block.extract();
        let gen_block_hash = block.get_hash();

        let blockchain = make_blockchain(gen_block).await;

        let gen_block = match blockchain.get_block(gen_block_hash).await {
            Ok(b) => b,
            Err(err) => panic!("Error : {}", err),
        };

        let get_gen_hash = gen_block.get_hash();
        let gen_tx_hashes = gen_block.get_tx_hashes();
        assert_ne!(get_gen_hash, &String::from("false hash"));

        for tx_hash in gen_tx_hashes {
            let tx = match blockchain.get_transaction(tx_hash).await {
                Ok(t) => t,
                Err(err) => panic!("Error : {}", err),
            };

            assert_eq!(tx_hash, tx.get_hash());
        }
    }
}
