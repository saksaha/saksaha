#[cfg(test)]
mod test {
    use aes::cipher::typenum::Diff;
    use sak_dist_ledger::{DistLedger, DistLedgerArgs};
    use sak_types::BlockCandidate;
    use sak_types::Hashable;
    use sak_types::Tx;

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
                    Some(vec![11, 11, 11]),
                ),
                Tx::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    b"2".to_vec(),
                    Some(vec![22, 22, 22]),
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: String::from("0"),
        };

        genesis_block
    }

    async fn make_blockchain(gen_block: BlockCandidate) -> DistLedger {
        let blockchain_args = DistLedgerArgs {
            app_prefix: String::from("test"),
            tx_pool_sync_interval: None,
        };

        let blockchain = DistLedger::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    fn make_dummy_txs() -> Vec<Tx> {
        vec![
            Tx::new(
                String::from("1346546123"),
                String::from("one").as_bytes().to_vec(),
                String::from("0x111"),
                b"0x1111".to_vec(),
                Some(String::from("one").as_bytes().to_vec()),
            ),
            Tx::new(
                String::from("1346546124"),
                String::from("two").as_bytes().to_vec(),
                String::from("0x222"),
                b"0x2222".to_vec(),
                Some(String::from("two").as_bytes().to_vec()),
            ),
            Tx::new(
                String::from("1346546125"),
                String::from("three").as_bytes().to_vec(),
                String::from("0x333"),
                b"0x3333".to_vec(),
                Some(String::from("three").as_bytes().to_vec()),
            ),
            Tx::new(
                String::from("1346546126"),
                String::from("four").as_bytes().to_vec(),
                String::from("0x444"),
                b"0x4444".to_vec(),
                Some(String::from("four").as_bytes().to_vec()),
            ),
        ]
    }
}
