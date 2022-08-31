use crate::{
    BlockCandidate, PourTxCandidate, Tx, TxCandidate, WASM_MAGIC_NUMBER,
};
// use sak::{
//     Consensus, ConsensusError, DistLedger, DistLedgerApis, DistLedgerArgs,
// };
// use async_trait::async_trait;
// use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{rand, Hasher, Scalar, ScalarExt};
use sak_proofs::{CoinProof, MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use std::collections::HashMap;
use type_extension::U8Array;

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

pub(crate) async fn mock_valid_pour_tx_candidate() -> TxCandidate {
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

    let pi =
        CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
            .unwrap();

    let pi_ser = CoinProof::serialize_pi(&pi).unwrap();

    PourTxCandidate::mock_tx_candidate(
        pi_ser,
        proof_context.sn_1.to_bytes(),
        proof_context.cm_1.to_bytes(),
        proof_context.cm_2.to_bytes(),
        proof_context.merkle_rt.to_bytes(),
    )
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
        let addr_sk =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64)).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64)).unwrap();

        let s =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64)).unwrap();

        let rho =
            ScalarExt::parse_arr(&U8Array::from_int(rand() as u64)).unwrap();

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
