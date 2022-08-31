use crate::*;
use crate::{BlockCandidate, Tx, TxCandidate};

use super::proof::mock_valid_pour_tx_candidate;

pub(crate) async fn mock_genesis_block_canidate() -> BlockCandidate {
    let tx_mint_1 = mock_mint_tc_3();
    let tx_mint_2 = mock_mint_tc_4();
    let tx_deploy_validator = mock_mint_tc_deploying_contract(
        VALIDATOR.to_vec(),
        VALIDATOR_CTR_ADDR.to_string(),
    );
    let tx_deploy_envelope = mock_mint_tc_deploying_contract(
        ENVELOPE.to_vec(),
        ENVELOPE_CTR_ADDR.to_string(),
    );
    let tx_pour_1 = mock_valid_pour_tx_candidate().await;
    let tx_pour_2 = mock_valid_pour_tx_candidate().await;

    let genesis_block = BlockCandidate {
        validator_sig: String::from("Ox16a03c8sbfaf3cb06"),
        tx_candidates: vec![
            tx_mint_1,
            tx_mint_2,
            tx_deploy_validator,
            tx_deploy_envelope,
            tx_pour_1,
            tx_pour_2,
        ],
        witness_sigs: vec![String::from("2"), String::from("3")],
        created_at: String::from("20220831"),
    };

    genesis_block
}
