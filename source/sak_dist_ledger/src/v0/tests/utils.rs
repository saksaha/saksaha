use crate::{
    Consensus, ConsensusError, DistLedger, DistLedgerApis, DistLedgerArgs,
};
use async_trait::async_trait;
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{rand, Hasher, Scalar, ScalarExt};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::{
    BlockCandidate, PourTxCandidate, Tx, TxCandidate, WASM_MAGIC_NUMBER,
};
use saksaha::generate_proof_1_to_2;
use std::collections::HashMap;
use type_extension::U8Array;

pub struct DummyPos {}

#[async_trait]
impl Consensus for DummyPos {
    async fn do_consensus(
        &self,
        _dist_ledger_apis: &DistLedgerApis,
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
    vec![Tx::new_dummy_pour_m1_to_p3_p4()]
}

pub(crate) async fn make_dummy_valid_pour_tx() -> Tx {
    let proof_context = make_proof_context();

    let coin_1_old = OldCoin {
        addr_pk: Some(proof_context.addr_pk_1_old),
        addr_sk: Some(proof_context.addr_sk_1_old),
        rho: Some(proof_context.rho_1_old),
        r: Some(proof_context.r_1_old),
        s: Some(proof_context.s_1_old),
        v: Some(proof_context.v_1_old),
        cm: Some(proof_context.cm_1_old),
        auth_path: proof_context.auth_path_1,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(proof_context.addr_pk_1),
        rho: Some(proof_context.rho_1),
        r: Some(proof_context.r_1),
        s: Some(proof_context.s_1),
        v: Some(proof_context.v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(proof_context.addr_pk_2),
        rho: Some(proof_context.rho_2),
        r: Some(proof_context.r_2),
        s: Some(proof_context.s_2),
        v: Some(proof_context.v_2),
    };

    let pi = generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
        .await
        .unwrap();

    let mut pi_ser = Vec::new();
    pi.write(&mut pi_ser).unwrap();

    {
        println!("\n[+] dummy pour_tx ");
        println!("[Debug] tx.pi: {:?}", pi);
        println!("[Debug] tx.sn_1: {:?}", proof_context.sn_1.clone(),);
        println!("[Debug] tx.cm_1: {:?}", proof_context.cm_1.clone(),);
        println!("[Debug] tx.cm_2: {:?}", proof_context.cm_2.clone(),);
        println!(
            "[Debug] tx.merkle_rt: {:?}",
            proof_context.merkle_rt.clone(),
        );
    }

    Tx::new_dummy_valid_pour(
        pi_ser,
        proof_context.sn_1.to_bytes(),
        proof_context.cm_1.to_bytes(),
        proof_context.cm_2.to_bytes(),
        proof_context.merkle_rt.to_bytes(),
    )
}

pub struct ProofContext {
    pub hasher: Hasher,

    // old coin 1
    pub addr_pk_1_old: Scalar,
    pub addr_sk_1_old: Scalar,
    pub r_1_old: Scalar,
    pub s_1_old: Scalar,
    pub rho_1_old: Scalar,
    pub v_1_old: Scalar,
    pub cm_1_old: Scalar,
    pub auth_path_1: [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
    pub merkle_rt: Scalar,
    pub sn_1: Scalar,

    // new coin 1
    pub addr_sk_1: Scalar,
    pub addr_pk_1: Scalar,
    pub r_1: Scalar,
    pub s_1: Scalar,
    pub rho_1: Scalar,
    pub v_1: Scalar,
    pub cm_1: Scalar,

    // new coin 2
    pub addr_sk_2: Scalar,
    pub addr_pk_2: Scalar,
    pub r_2: Scalar,
    pub s_2: Scalar,
    pub rho_2: Scalar,
    pub v_2: Scalar,
    pub cm_2: Scalar,
}
pub(crate) fn make_proof_context() -> ProofContext {
    let hasher = Hasher::new();

    let (
        addr_pk_1_old,
        addr_sk_1_old,
        r_1_old,
        s_1_old,
        rho_1_old,
        v_1_old,
        cm_1_old,
        sn_1,
    ) = {
        let addr_sk = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let rho = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(1000)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
        let addr_sk =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
                .unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();
        let rho = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(600)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
                .unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();

        let rho = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100))
            .unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(400)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

    let merkle_nodes = {
        let mut m = HashMap::new();

        let node_0_1 = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
        let node_1_1 = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
        let node_2_1 = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
        let node_3_1 = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();
        // let node_4_1 = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();

        m.insert("0_1", node_0_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_1", node_3_1);
        // m.insert("4_1", node_4_1);

        let node_1_0 = hasher.mimc_scalar(cm_1_old, node_0_1);
        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);
        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);
        let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);
        // let node_5_0 = hasher.mimc_scalar(node_4_0, node_4_1);

        m.insert("1_0", node_1_0);
        m.insert("2_0", node_2_0);
        m.insert("3_0", node_3_0);
        m.insert("4_0", node_4_0);
        // m.insert("5_0", node_5_0);

        m
    };

    let merkle_rt = *merkle_nodes.get("4_0").unwrap();

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);
        let mut ret =
            [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!(
                    "Invalid assignment to a fixed sized array, idx: {}",
                    idx
                );
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = merkle_nodes.get(key.as_str()).unwrap();

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    ProofContext {
        hasher,
        //
        addr_pk_1_old,
        addr_sk_1_old,
        r_1_old,
        s_1_old,
        rho_1_old,
        v_1_old,
        cm_1_old,
        auth_path_1,
        merkle_rt,
        sn_1,
        //
        addr_sk_1,
        addr_pk_1,
        r_1,
        s_1,
        rho_1,
        v_1,
        cm_1,
        //
        addr_sk_2,
        addr_pk_2,
        r_2,
        s_2,
        rho_2,
        v_2,
        cm_2,
    }
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
    let block_candidate: BlockCandidate = {
        BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            tx_candidates: vec![TxCandidate::new_dummy_pour_m1_to_p3_p4()],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
        }
    };

    Some(block_candidate)
}

pub(crate) fn make_dummy_block_candidate_with_query_tx(
) -> Option<BlockCandidate> {
    let block_candidate: BlockCandidate = {
        let dummy_ctr_calling_query_tc: TxCandidate = {
            let request_query_get_validator: CtrRequest = {
                CtrRequest {
                    req_type: "get_validator".to_string(),
                    args: vec![],
                    ctr_call_type: CtrCallType::Query,
                }
            };

            TxCandidate::Pour(PourTxCandidate::new(
                String::from("created_at_1"),
                WASM_MAGIC_NUMBER.to_vec(),
                String::from("author_sig_1"),
                Some(String::from("ctr_addr_1")),
                vec![0],
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
        }
    };

    Some(block_candidate)
}

// pub(crate) fn make_dummy_block_candidate_calling_validator_ctr(
// ) -> Option<BlockCandidate> {
//     let block_candidate = {
//         let dummy_validator_1 = String::from(
//             "\
//                     aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
//                     bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
//                     ccccccccccccccccccccccccccccccccc\
//                     2222222222222222222222222222222\
//                 ",
//         );

//         let mut args = HashMap::with_capacity(10);
//         args.insert(String::from("validator"), dummy_validator_1);

//         let request_execute_add_validator_1 = Request {
//             req_type: "add_validator".to_string(),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let data = [
//             // &WASM_MAGIC_NUMBER,
//             serde_json::to_string(&request_execute_add_validator_1)
//                 .unwrap()
//                 .as_bytes(),
//         ]
//         .concat();

//         let dummy_ctr_calling_execute_add_validator_tc_1 =
//             TxCandidate::Pour(PourTxCandidate::new(
//                 String::from("created_at_1"),
//                 data.to_vec(),
//                 String::from("author_sig_1"),
//                 Some(String::from("test_validator_1")),
//                 vec![22],
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//             ));

//         let dummy_validator_2 = String::from(
//             "\
//                     aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
//                     bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
//                     ccccccccccccccccccccccccccccccccc\
//                     3333333333333333333333333333333\
//                 ",
//         );

//         let mut args = HashMap::with_capacity(10);
//         args.insert(String::from("validator"), dummy_validator_2);

//         let request_execute_add_validator_2 = Request {
//             req_type: "add_validator".to_string(),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let data = [serde_json::to_string(&request_execute_add_validator_2)
//             .unwrap()
//             .as_bytes()]
//         .concat();

//         let dummy_ctr_calling_execute_add_validator_tc_2 =
//             TxCandidate::Pour(PourTxCandidate::new(
//                 String::from("created_at_2"),
//                 data.to_vec(),
//                 String::from("author_sig_2"),
//                 Some(String::from("test_validator_1")),
//                 vec![22],
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//                 U8Array::new_empty_32(),
//             ));

//         BlockCandidate {
//             validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
//             tx_candidates: vec![
//                 dummy_ctr_calling_execute_add_validator_tc_1,
//                 dummy_ctr_calling_execute_add_validator_tc_2,
//             ],
//             witness_sigs: vec![String::from("3"), String::from("4")],
//             created_at: String::from("2022061515340000"),
//         }
//     };

//     Some(block_candidate)
// }
