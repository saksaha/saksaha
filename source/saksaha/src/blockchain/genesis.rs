use sak_types::{BlockCandidate, Transaction};

pub(super) fn make_genesis_block() -> BlockCandidate {
    let ctrt_bytes =
        include_bytes!("../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

    let genesis_block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        transactions: vec![
            Transaction::new(
                String::from("1"),
                ctrt_bytes.to_vec(),
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
