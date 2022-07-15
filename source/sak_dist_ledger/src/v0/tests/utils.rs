use crate::{Consensus, ConsensusError, DistLedger, DistLedgerArgs};
use async_trait::async_trait;
use sak_contract_std::{CtrCallType, Request};
use sak_types::{BlockCandidate, PourTxCandidate, Tx, TxCandidate, U8Array};
use std::collections::HashMap;

pub struct DummyPos {}

#[async_trait]
impl Consensus for DummyPos {
    async fn do_consensus(
        &self,
        _dist_ledger: &DistLedger,
        _txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}

pub(crate) fn make_dummy_genesis_block_1() -> BlockCandidate {
    let genesis_block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![
            TxCandidate::new_dummy_mint_1(),
            TxCandidate::new_dummy_mint_2(),
            TxCandidate::new_dummy_pour_1_2_to_3(),
        ],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    genesis_block
}

pub(crate) async fn make_dist_ledger() -> DistLedger {
    let pos = make_dummy_pos();

    let dist_ledger_args = DistLedgerArgs {
        app_prefix: String::from("test"),
        tx_sync_interval: None,
        genesis_block: Some(make_dummy_genesis_block_1()),
        consensus: pos,
        block_sync_interval: None,
    };

    let dist_ledger = DistLedger::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub(crate) fn make_dummy_txs() -> Vec<Tx> {
    vec![
        Tx::new_dummy_pour_1_2_to_3(),
        Tx::new_dummy_pour_2(),
        Tx::new_dummy_pour_3(),
        Tx::new_dummy_pour_4(),
    ]
}

pub(crate) fn make_dummy_state() -> (String, String) {
    let contract_addr = String::from("0xa1a2a3a4");
    let ctr_state = String::from("test_ctr_state");

    (contract_addr, ctr_state)
}

pub(crate) fn make_dummy_pos() -> Box<DummyPos> {
    Box::new(DummyPos {})
}

#[cfg(test)]
pub(crate) fn make_dummy_block_candidate_1() -> Option<BlockCandidate> {
    // let test_wasm = include_bytes!("./test_valid_contract.wasm").to_vec();

    let block_candidate: BlockCandidate = {
        let dummy_ctr_deploying_tc = TxCandidate::new_dummy_pour_1_2_to_3();

        BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![dummy_ctr_deploying_tc],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            // block_height: 0,
            // merkle_root: String::from("2022061515340000"),
        }
    };

    Some(block_candidate)
}

pub(crate) fn make_dummy_block_candidate_with_query_tx(
) -> Option<BlockCandidate> {
    let block_candidate: BlockCandidate = {
        let dummy_ctr_calling_query_tc: TxCandidate = {
            let request_query_get_validator: Request = {
                Request {
                    req_type: "get_validator".to_string(),
                    arg: HashMap::with_capacity(10),
                    ctr_call_type: CtrCallType::Query,
                }
            };

            TxCandidate::Pour(PourTxCandidate::new(
                String::from("created_at_0"),
                serde_json::to_string(&request_query_get_validator)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig_0"),
                Some(String::from("ctr_addr_0")),
                vec![0],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            ))
        };

        BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![dummy_ctr_calling_query_tc],
            witness_sigs: vec![String::from("3"), String::from("4")],
            created_at: String::from("2022061515340000"),
            // block_height: 1,
            // merkle_root: String::from("2022061515340000"),
        }
    };

    Some(block_candidate)
}

pub(crate) fn make_dummy_block_candidate_with_execute_tx(
) -> Option<BlockCandidate> {
    let block_candidate = {
        let dummy_validator_1 = String::from(
            "\
                    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
                    ccccccccccccccccccccccccccccccccc\
                    2222222222222222222222222222222\
                ",
        );

        let mut arg = HashMap::with_capacity(10);
        arg.insert(String::from("validator"), dummy_validator_1);

        let request_execute_add_validator_1 = Request {
            req_type: "add_validator".to_string(),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        let dummy_ctr_calling_execute_add_validator_tc_1 =
            TxCandidate::Pour(PourTxCandidate::new(
                String::from("created_at_1"),
                serde_json::to_string(&request_execute_add_validator_1)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig_1"),
                Some(String::from("ctr_addr_1")),
                vec![22],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            ));

        let dummy_validator_2 = String::from(
            "\
                    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
                    ccccccccccccccccccccccccccccccccc\
                    3333333333333333333333333333333\
                ",
        );

        let mut arg = HashMap::with_capacity(10);
        arg.insert(String::from("validator"), dummy_validator_2);

        let request_execute_add_validator_2 = Request {
            req_type: "add_validator".to_string(),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        let dummy_ctr_calling_execute_add_validator_tc_2 =
            TxCandidate::Pour(PourTxCandidate::new(
                String::from("created_at_2"),
                serde_json::to_string(&request_execute_add_validator_2)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig_2"),
                Some(String::from("ctr_addr_2")),
                vec![22],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            ));

        BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![
                //
                dummy_ctr_calling_execute_add_validator_tc_1,
                dummy_ctr_calling_execute_add_validator_tc_2,
            ],
            witness_sigs: vec![String::from("3"), String::from("4")],
            created_at: String::from("2022061515340000"),
            // block_height: 2,
            // merkle_root: String::from("2022061515340000"),
        }
    };

    Some(block_candidate)
}
