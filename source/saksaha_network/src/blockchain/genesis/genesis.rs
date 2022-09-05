use crate::SaksahaError;
use sak_proofs::Hasher;
use sak_types::{BlockCandidate, TxCandidate};

pub(crate) const VALIDATOR_SIG: &str = "validator_sig";

pub(crate) const VALIDATOR_CTR_ADDR: &'static str = "validator_contract_addr";

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../../prebuild/sak_validator.postprocess.wasm");

pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";

pub(crate) const ENVELOPE: &[u8] =
    include_bytes!("../../../../prebuild/envelope_contract.postprocess.wasm");

pub(crate) struct GenesisBlock {
    pub(crate) block_candidate: BlockCandidate,
}

impl GenesisBlock {
    pub fn create() -> Result<GenesisBlock, SaksahaError> {
        let validator_wasm = VALIDATOR.to_vec();
        let envelope_wasm = ENVELOPE.to_vec();

        let tx_mint_1 = sak_types::mock_mint_tc_3();
        let tx_mint_2 = sak_types::mock_mint_tc_4();

        let tx_mint_dummy_old_coin = sak_types::mock_mint_tc_dummy_old_coin();

        let tx_deploy_validator = sak_types::mock_mint_tc_deploying_contract(
            validator_wasm,
            VALIDATOR_CTR_ADDR.to_string(),
        );
        let tx_deploy_envelope = sak_types::mock_mint_tc_deploying_contract(
            envelope_wasm,
            ENVELOPE_CTR_ADDR.to_string(),
        );
        let tx_mint_3 = sak_types::mock_mint_tc_5();
        let tx_mint_4 = sak_types::mock_mint_tc_6();

        let block_candidate = BlockCandidate {
            validator_sig: VALIDATOR_SIG.to_string(),
            tx_candidates: vec![
                tx_mint_1,
                tx_mint_2,
                tx_mint_dummy_old_coin,
                tx_deploy_validator,
                tx_deploy_envelope,
                tx_mint_3,
                tx_mint_4,
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
        };

        let gen_block = GenesisBlock { block_candidate };

        Ok(gen_block)
    }

    pub fn get_validator_ctr_addr(&self) -> String {
        VALIDATOR_CTR_ADDR.to_string()
    }
}
