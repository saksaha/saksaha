use sak_crypto::{Hasher, Scalar, ScalarExt};
use sak_types::{BlockCandidate, MintTxCandidate, Tx, TxCandidate, U8Array};

use crate::system::BoxedError;

pub(crate) const VALIDATOR_SIG: &str = "validator_sig";

pub(crate) const VALIDATOR_CTR_ADDR: &'static str = "validator_contract_addr";

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

pub(crate) struct GenesisBlock {
    pub(crate) block_candidate: BlockCandidate,
}

impl GenesisBlock {
    pub fn create() -> Result<GenesisBlock, BoxedError> {
        let validator_wasm = VALIDATOR.to_vec();

        let hasher = Hasher::new();

        let mint_tx_1 = {
            let v = Scalar::from(1000);

            let s = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let r = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let rho = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let a_pk = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let k = hasher.comm2(r, a_pk, rho);

            let cm = hasher.comm2(s, v, k);

            TxCandidate::Mint(MintTxCandidate::new(
                String::from("initial_mint_created_at"),
                vec![0],
                VALIDATOR_SIG.to_string(),
                None,
                cm.to_bytes(),
                v.to_bytes(),
                k.to_bytes(),
                s.to_bytes(),
            ))
        };

        let validator_deploy_tx = {
            let v = Scalar::from(1000);

            let s = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let r = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let rho = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let a_pk = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr)?
            };

            let k = hasher.comm2(r, a_pk, rho);

            let cm = hasher.comm2(s, v, k);

            TxCandidate::Mint(MintTxCandidate::new(
                String::from("initial_mint_created_at"),
                validator_wasm,
                VALIDATOR_SIG.to_string(),
                Some(VALIDATOR_CTR_ADDR.to_string()),
                cm.to_bytes(),
                v.to_bytes(),
                k.to_bytes(),
                s.to_bytes(),
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
