use sak_contract_std::{CtrCallType, Request};
use sak_types::{BlockCandidate, Tx, TxCandidate};
use std::collections::HashMap;

pub(crate) fn make_dummy_block_candidate_1() -> Option<BlockCandidate> {
    let test_wasm = include_bytes!("./test_valid_contract.wasm").to_vec();

    let block_candidate: BlockCandidate = {
        let dummy_ctr_deploying_tc = TxCandidate::new(
            String::from("1"),
            vec![11, 11, 11],
            String::from("1"),
            Some(b"1".to_vec()),
            None,
            None,
            Some(String::from("v")),
            Some(String::from("k")),
            Some(String::from("s")),
            Some(String::from("sn_1")),
            Some(String::from("sn_2")),
            Some(vec![1]),
            Some(vec![1]),
            Some(String::from("rt")),
        );

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

            TxCandidate::new(
                String::from("created_at0"),
                serde_json::to_string(&request_query_get_validator)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig0"),
                Some(vec![0]), // pi
                Some(String::from("ctr_addr0")),
                None,
                Some(String::from("v")),
                Some(String::from("k")),
                Some(String::from("s")),
                Some(String::from("sn_1")),
                Some(String::from("sn_2")),
                Some(vec![1]),
                Some(vec![1]),
                Some(String::from("rt")),
            )
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

        let dummy_ctr_calling_execute_add_validator_tc_1 = {
            TxCandidate::new(
                String::from("created_at1"),
                serde_json::to_string(&request_execute_add_validator_1)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig1"),
                Some(vec![1]), // pi
                Some(String::from("ctr_addr1")),
                None,
                Some(String::from("v")),
                Some(String::from("k")),
                Some(String::from("s")),
                Some(String::from("sn_1")),
                Some(String::from("sn_2")),
                Some(vec![1]),
                Some(vec![1]),
                Some(String::from("rt")),
            )
        };

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

        let dummy_ctr_calling_execute_add_validator_tc_2 = {
            TxCandidate::new(
                String::from("created_at2"),
                serde_json::to_string(&request_execute_add_validator_2)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                String::from("author_sig2"),
                Some(vec![2]), // pi
                Some(String::from("ctr_addr2")),
                None,
                Some(String::from("v")),
                Some(String::from("k")),
                Some(String::from("s")),
                Some(String::from("sn_1")),
                Some(String::from("sn_2")),
                Some(vec![1]),
                Some(vec![1]),
                Some(String::from("rt")),
            )
        };

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