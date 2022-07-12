use sak_proofs::Hasher;
use sak_types::{BlockCandidate, MintTxCandidate, Tx, TxCandidate};

pub(crate) const VALIDATOR_SIG: &str = "validator_sig";

pub(crate) const VALIDATOR_CTR_ADDR: &'static str = "validator_contract_addr";

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

pub(crate) struct GenesisBlock {
    pub(crate) block_candidate: BlockCandidate,
}

impl GenesisBlock {
    pub fn create() -> GenesisBlock {
        let validator_wasm = VALIDATOR.to_vec();

        // created_at: String,
        // data: Vec<u8>,
        // author_sig: String,
        // ctr_addr: Option<String>,
        // cm: Vec<u8>,
        // v: String,
        // k: String,
        // s: String,

        let v = 100_000;
        let k = 13;
        let s = 24;

        let hasher = Hasher::new();
        let cm = hasher.comm2(s, hasher.prf(v, k)).to_bytes();

        let mint_tx_1 = TxCandidate::Mint(MintTxCandidate::new(
            String::from("initial_mint_created_at"),
            vec!["initial_mint_data"],
            VALIDATOR_SIG,
            None,
        ));

        let validator_deploy_tx = TxCandidate::new(
            String::from("1"),
            validator_wasm,
            String::from("1"),
            vec![1],
            Some(VALIDATOR_CTR_ADDR.to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let block_candidate = BlockCandidate {
            validator_sig: VALIDATOR_SIG.to_string(),
            tx_candidates: vec![validator_deploy_tx, some_other_tx],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
        };

        GenesisBlock { block_candidate }
    }

    pub fn get_validator_ctr_addr(&self) -> String {
        VALIDATOR_CTR_ADDR.to_string()
    }
}
