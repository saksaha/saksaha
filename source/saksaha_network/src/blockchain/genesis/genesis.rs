use crate::SaksahaError;
use sak_crypto::Hasher;
use sak_types::{BlockCandidate, MintTxCandidate, TxCandidate, U8Array};

pub(crate) const VALIDATOR_SIG: &str = "validator_sig";

pub(crate) const VALIDATOR_CTR_ADDR: &'static str = "validator_contract_addr";

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

pub(crate) struct GenesisBlock {
    pub(crate) block_candidate: BlockCandidate,
}

impl GenesisBlock {
    pub fn create() -> Result<GenesisBlock, SaksahaError> {
        let validator_wasm = VALIDATOR.to_vec();

        let hasher = Hasher::new();

        let mint_tx_1 = {
            let v = U8Array::new_empty_32();

            let s = U8Array::new_empty_32();

            let r = U8Array::new_empty_32();

            let rho = U8Array::new_empty_32();

            let a_pk = U8Array::new_empty_32();

            let k = hasher.comm2(&r, &a_pk, &rho)?;

            let cm = hasher.comm2(&s, &v, &k.to_bytes())?;

            TxCandidate::Mint(MintTxCandidate::new(
                String::from("initial_mint_created_at"),
                vec![0],
                VALIDATOR_SIG.to_string(),
                None,
                cm.to_bytes(),
                v,
                k.to_bytes(),
                s,
            ))
        };

        let validator_deploy_tx = {
            let v = U8Array::new_empty_32();

            let s = U8Array::new_empty_32();

            let r = U8Array::new_empty_32();

            let rho = U8Array::new_empty_32();

            let a_pk = U8Array::new_empty_32();

            let k = hasher.comm2(&r, &a_pk, &rho)?;

            let cm = hasher.comm2(&s, &v, &k.to_bytes())?;

            TxCandidate::Mint(MintTxCandidate::new(
                String::from("initial_mint_created_at"),
                validator_wasm,
                VALIDATOR_SIG.to_string(),
                Some(VALIDATOR_CTR_ADDR.to_string()),
                cm.to_bytes(),
                v,
                k.to_bytes(),
                s,
            ))
        };

        let block_candidate = BlockCandidate {
            validator_sig: VALIDATOR_SIG.to_string(),
            tx_candidates: vec![mint_tx_1, validator_deploy_tx],
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
