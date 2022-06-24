use sak_types::{Block, BlockCandidate, Tx};

pub(crate) struct GenesisBlock {
    // pub(crate) validator_contract_addr: &'static str,
    pub(crate) block_candidate: BlockCandidate,
}

pub(crate) const VALIDATOR_SIG: &str = "alw";

pub(crate) const VALIDATOR_CONTRACT_ADDR: &'static str = "alwekfj";

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

impl GenesisBlock {
    pub fn create() -> BlockCandidate {
        let validator_wasm = VALIDATOR.to_vec();

        let validator_deploy_tx = Tx::new(
            String::from("1"),
            validator_wasm,
            String::from("1"),
            String::from("1"),
            vec![1],
        );

        let some_other_tx = Tx::new(
            String::from("2"),
            vec![22, 22, 22],
            String::from("2"),
            String::from("2"),
            vec![1],
        );

        let genesis_block = BlockCandidate {
            validator_sig: VALIDATOR_SIG.to_string(),
            transactions: vec![validator_deploy_tx, some_other_tx],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: String::from("0"),
        };

        genesis_block
    }
}
