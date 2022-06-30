use crate::DistLedger;
use crate::{Consensus, ConsensusError};
use async_trait::async_trait;
use sak_types::{BlockCandidate, Tx};

mod utils;

#[cfg(test)]
mod test {
    use super::utils;
    use super::Pos;
    use crate::SyncPool;
    use crate::{DistLedger, DistLedgerArgs};
    use sak_contract_std::Storage;
    use sak_types::{BlockCandidate, Tx};

    fn init() {
        let _ = env_logger::builder().is_test(true).init();
    }

    fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            transactions: vec![
                Tx::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    b"1".to_vec(),
                    Some(String::from("11")),
                ),
                Tx::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    b"2".to_vec(),
                    Some(String::from("22")),
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: 0,
        };

        genesis_block
    }

    async fn make_dist_ledger() -> DistLedger {
        let pos = Box::new(Pos {});

        let dist_ledger_args = DistLedgerArgs {
            app_prefix: String::from("test"),
            tx_sync_interval: None,
            genesis_block: None,
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
            Tx::new(
                String::from("1346546123"),
                String::from("one").as_bytes().to_vec(),
                String::from("0x111"),
                b"0x1111".to_vec(),
                Some(String::from("one")),
            ),
            Tx::new(
                String::from("1346546124"),
                String::from("two").as_bytes().to_vec(),
                String::from("0x222"),
                b"0x2222".to_vec(),
                Some(String::from("two")),
            ),
            Tx::new(
                String::from("1346546125"),
                String::from("three").as_bytes().to_vec(),
                String::from("0x333"),
                b"0x3333".to_vec(),
                Some(String::from("three")),
            ),
            Tx::new(
                String::from("1346546126"),
                String::from("four").as_bytes().to_vec(),
                String::from("0x444"),
                b"0x4444".to_vec(),
                Some(String::from("four")),
            ),
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

        let gen_block = make_dummy_genesis_block();
        let blockchain = make_dist_ledger().await;

        let db = blockchain.ledger_db;

        let dummy_tx_values = make_dummy_txs();
        let mut tx_hashes = vec![];

        for tx_val in dummy_tx_values.iter() {
            let h = db.put_tx(&tx_val).expect("Tx should be written");

            tx_hashes.push(h);
        }

        for (idx, tx_hash) in tx_hashes.iter().enumerate() {
            let tx_val_retrieved =
                db.get_tx(tx_hash).await.expect("Tx should exist");

            assert_eq!(
                tx_val_retrieved.unwrap().get_data(),
                dummy_tx_values[idx].get_data()
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_wrongful_put_and_get_transaction() {
        init();

        let gen_block = make_dummy_genesis_block();
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

        let gen_block = make_dummy_genesis_block();
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
    async fn test_insert_genesis_block_and_check_tx() {
        init();

        let gen_block = make_dummy_genesis_block();

        let gen_block_same = make_dummy_genesis_block();

        let (block, _) = gen_block_same.extract();

        let gen_block_hash = block.get_hash();

        let blockchain = make_dist_ledger().await;

        // let gen_block_hash = blockchain
        //     .get_gen_block_hash()
        //     .as_ref()
        //     .expect("Genesis block should have been inserted");

        let gen_block_by_height = match blockchain.get_block_by_height(&0).await
        {
            Ok(b) => match b {
                Some(b) => b,
                None => {
                    panic!("cannot find genesis block");
                }
            },
            Err(err) => panic!("Error : {}", err),
        };

        let gen_block_hash_2 = gen_block_by_height.get_hash();

        assert_eq!(gen_block_hash, gen_block_hash_2);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_insert_genesis_block_and_check_wrong_block_hash() {
        init();

        let gen_block = make_dummy_genesis_block();
        let dist_ledger = make_dist_ledger().await;

        let gen_block = dist_ledger
            .get_block_by_height(&0)
            .await
            .unwrap()
            .expect("gen block should exist");

        let get_gen_hash = gen_block.get_hash();
        let gen_tx_hashes = gen_block.get_tx_hashes();

        for tx_hash in gen_tx_hashes {
            let tx = match dist_ledger.get_tx(tx_hash).await {
                Ok(t) => t,
                Err(err) => panic!("Error : {}", err),
            };

            let tx = tx.unwrap();

            assert_eq!(tx_hash, tx.get_hash());
        }

        assert_ne!(get_gen_hash, &String::from("false hash"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_set_and_get_contract_state_to_db() {
        init();

        let gen_block = make_dummy_genesis_block();
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

        let dummy_tx = Tx::new(
            String::from("1346546123"),
            test_wasm,
            String::from("0x111"),
            b"0x1111".to_vec(),
            Some(String::from("test_wasm")),
        );

        let sync_pool = SyncPool::new();

        sync_pool.insert_tx(dummy_tx).await.unwrap();
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

            // let expected_state: Storage = Storage::new();
            // expected_state.insert("validators", v)
            // assert_eq!(result, Storage{});
        }
    }
}

pub struct Pos {}

#[async_trait]
impl Consensus for Pos {
    async fn do_consensus(
        &self,
        _dist_ledger: &DistLedger,
        _txs: Vec<Tx>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}
