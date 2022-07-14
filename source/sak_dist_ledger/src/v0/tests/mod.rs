mod utils;

use crate::DistLedger;
use crate::{Consensus, ConsensusError};
use async_trait::async_trait;
use sak_types::{BlockCandidate, TxCandidate};

#[cfg(test)]
mod test {
    use super::utils;
    use super::Pos;
    use crate::SyncPool;
    use crate::{DistLedger, DistLedgerArgs};
    use sak_contract_std::Storage;
    use sak_types::PourTx;
    use sak_types::PourTxCandidate;
    use sak_types::{BlockCandidate, Tx, TxCandidate};

    const RUST_LOG_ENV: &str = "
        sak_,
        saksaha
    ";

    pub fn init() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        sak_logger::init(false);
    }

    fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![
                TxCandidate::new_dummy_mint_1(),
                TxCandidate::new_dummy_mint_2(),
                TxCandidate::new_dummy_pour_1(),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
        };

        genesis_block
    }

    async fn make_dist_ledger() -> DistLedger {
        let pos = Box::new(Pos {});

        let dist_ledger_args = DistLedgerArgs {
            app_prefix: String::from("test"),
            tx_sync_interval: None,
            genesis_block: Some(make_dummy_genesis_block()),
            consensus: pos,
            block_sync_interval: None,
        };

        let dist_ledger = DistLedger::init(dist_ledger_args)
            .await
            .expect("Blockchain should be initialized");

        dist_ledger
    }

    fn make_dummy_txs() -> Vec<Tx> {
        vec![
            Tx::new_dummy_pour_1(),
            Tx::new_dummy_pour_2(),
            Tx::new_dummy_pour_3(),
            Tx::new_dummy_pour_4(),
        ]
    }

    fn make_dummy_state() -> (String, String) {
        let contract_addr = String::from("0xa1a2a3a4");
        let ctr_state = String::from("test_ctr_state");

        (contract_addr, ctr_state)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_and_get_transaction() {
        init();
        sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

        let blockchain = make_dist_ledger().await;

        let db = blockchain.ledger_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = db.put_tx(&tx_val).expect("Tx should be written");

            tx_hashes.push(h);
        }

        for (idx, tx_hash) in tx_hashes.iter().enumerate() {
            let tx_val_retrieved = db
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
    async fn test_wrongful_put_and_get_transaction() {
        init();

        let blockchain = make_dist_ledger().await;
        let db = blockchain.ledger_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = db.put_tx(&tx_val).expect("Tx should be written");

            tx_hashes.push(h);
        }

        let target_idx = 0;
        let wrong_idx = 1;

        let tx_val_retrieved = db
            .get_tx(&tx_hashes[target_idx])
            .await
            .expect("Tx should exist");

        assert_ne!(
            tx_val_retrieved.unwrap().get_data(),
            dummy_tx_values[wrong_idx].get_data()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn raw_iterator_to_first() {
        init();

        let blockchain = make_dist_ledger().await;
        let db = blockchain.ledger_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = db.put_tx(&tx_val).expect("Tx should be written");

            tx_hashes.push(h);
        }

        let mut iter = db.kv_db.db_instance.raw_iterator_cf(
            &db.kv_db.db_instance.cf_handle("created_at").unwrap(),
        );

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
    async fn test_insert_genesis_block_and_check_wrong_block_hash() {
        init();

        let dist_ledger = make_dist_ledger().await;

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
        init();

        let blockchain = make_dist_ledger().await;
        let db = blockchain.ledger_db;

        let (contract_addr, ctr_state) = make_dummy_state();

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

        let dummy_tx = TxCandidate::new_dummy_pour_1();

        let sync_pool = SyncPool::new();

        sync_pool.insert_tx(dummy_tx).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_write_a_genesis_block() {
        let dist_ledger = make_dist_ledger().await;

        dist_ledger.run().await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_deploy_ctr_when_dist_ledger_writes_a_new_block() {
        let dist_ledger = make_dist_ledger().await;

        dist_ledger.run().await;

        dist_ledger
            .write_block(utils::make_dummy_block_candidate_1())
            .await
            .expect("Block_1 must be written");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_deploy_ctr_and_invoke_query_when_dist_ledger_writes_new_blocks(
    ) {
        let dist_ledger = make_dist_ledger().await;

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

        let dist_ledger = make_dist_ledger().await;

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
        init();

        let blockchain = make_dist_ledger().await;

        for i in 0..10000 as u64 {
            let block = BlockCandidate {
                validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
                tx_candidates: vec![TxCandidate::new_dummy_pour_1()],
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
        init();

        let blockchain = make_dist_ledger().await;

        let repeat = 100;

        for i in 0..repeat as u64 {
            let block = BlockCandidate {
                validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
                tx_candidates: vec![
                    TxCandidate::new_dummy_pour_1(),
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

        let tx_height =
            blockchain.get_latest_tx_height().await.unwrap().unwrap();
        assert_eq!(2 * repeat - 1, tx_height);
    }
}

pub struct Pos {}

#[async_trait]
impl Consensus for Pos {
    async fn do_consensus(
        &self,
        _dist_ledger: &DistLedger,
        _txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}
