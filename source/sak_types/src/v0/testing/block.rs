use crate::BlockCandidate;

pub fn mock_block_1() -> BlockCandidate {
    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![crate::mock_mint_tc_1(), crate::mock_mint_tc_2()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    block
}

pub fn mock_block_2() -> BlockCandidate {
    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![crate::mock_mint_tc_3(), crate::mock_mint_tc_4()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    block
}

pub fn mock_block_pour_random() -> BlockCandidate {
    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![
            crate::mock_pour_tc_random(),
            crate::mock_pour_tc_random(),
        ],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    block
}

pub fn mock_block_pour_single() -> BlockCandidate {
    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![crate::mock_pour_tc_1()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    block
}

pub fn mock_block_invalid_pour() -> BlockCandidate {
    let block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![crate::mock_pour_tc_invalid_pi()],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    block
}
