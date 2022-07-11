mod p2p_block_sync;
mod p2p_marshal_tx_pool;
mod p2p_stream_cipher;

#[cfg(test)]
mod test {
    use sak_types::BlockCandidate;
    use sak_types::Tx;
    use sak_types::TxCandidate;

    fn init() {
        let _ = env_logger::builder().is_test(true).init();
    }

    fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![
                // TxCandidate::new(
                //     String::from("1"),
                //     vec![11, 11, 11],
                //     String::from("1"),
                //     b"1".to_vec(),
                //     Some(String::from("11")),
                //     Some(vec![11, 11, 11]),
                //     Some(String::from("11")),
                //     Some(String::from("11")),
                //     Some(String::from("11")),
                //     Some(String::from("11")),
                //     Some(String::from("11")),
                //     Some(vec![11, 11, 11]),
                //     Some(vec![11, 11, 11]),
                //     Some(String::from("11")),
                // ),
                TxCandidate::new_dummy_tx_candidate_1(),
                // TxCandidate::new(
                //     String::from("2"),
                //     vec![22, 22, 22],
                //     String::from("2"),
                //     b"2".to_vec(),
                //     Some(String::from("22")),
                //     Some(vec![22, 22, 22]),
                //     Some(String::from("22")),
                //     Some(String::from("22")),
                //     Some(String::from("22")),
                //     Some(String::from("22")),
                //     Some(String::from("22")),
                //     Some(vec![22, 22, 22]),
                //     Some(vec![22, 22, 22]),
                //     Some(String::from("22")),
                // ),,
                TxCandidate::new_dummy_tx_candidate_2(),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
        };

        genesis_block
    }

    fn make_dummy_txs() -> Vec<Tx> {
        vec![
            // Tx::new(
            //     String::from("1346546123"),
            //     String::from("one").as_bytes().to_vec(),
            //     String::from("0x000"),
            //     b"0x000".to_vec(),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![],
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![0],
            //     vec![0],
            //     String::from("one"),
            //     0,
            // ),
            Tx::new_dummy_tx_1(),
            // Tx::new(
            //     String::from("1346546123"),
            //     String::from("one").as_bytes().to_vec(),
            //     String::from("0x111"),
            //     b"0x1111".to_vec(),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![],
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![1],
            //     vec![1],
            //     String::from("one"),
            //     1,
            // ),
            Tx::new_dummy_tx_2(),
            // Tx::new(
            //     String::from("1346546123"),
            //     String::from("one").as_bytes().to_vec(),
            //     String::from("0x000"),
            //     b"0x000".to_vec(),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![],
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![2],
            //     vec![2],
            //     String::from("one"),
            //     2,
            // ),
            Tx::new_dummy_tx_3(),
            // Tx::new(
            //     String::from("1346546123"),
            //     String::from("one").as_bytes().to_vec(),
            //     String::from("0x000"),
            //     b"0x000".to_vec(),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![],
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     String::from("one"),
            //     vec![3],
            //     vec![3],
            //     String::from("one"),
            //     3,
            // ),
            Tx::new_dummy_tx_4(),
        ]
    }
}
