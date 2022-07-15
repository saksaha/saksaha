use sak_types::BlockCandidate;
use sak_types::Tx;
use sak_types::TxCandidate;

fn make_dummy_genesis_block() -> BlockCandidate {
    let genesis_block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![
            TxCandidate::new_dummy_pour_1_2_to_3(),
            TxCandidate::new_dummy_pour_2(),
        ],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    genesis_block
}

fn make_dummy_txs() -> Vec<Tx> {
    vec![
        Tx::new_dummy_pour_1_2_to_3(),
        Tx::new_dummy_pour_2(),
        Tx::new_dummy_pour_3(),
        Tx::new_dummy_pour_4(),
    ]
}
