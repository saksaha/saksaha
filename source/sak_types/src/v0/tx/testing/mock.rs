use super::TxCandidate;
use crate::TypesError;
use crate::{MintTxCandidate, PourTxCandidate, Tx, WASM_MAGIC_NUMBER};
use sak_crypto::Hasher;
use sak_crypto::ScalarExt;
use sak_crypto::{rand, Scalar};
use sak_proofs::CoinProof;
use sak_proofs::MerkleTree;
use sak_proofs::NewCoin;
use sak_proofs::OldCoin;
use sak_proofs::CM_TREE_DEPTH;
use std::collections::HashMap;
use type_extension::U8Arr32;
use type_extension::U8Array;

pub fn mock_pour_tc_1() -> Result<TxCandidate, TypesError> {
    // let hasher = Hasher::new();

    // let sn_1 = {
    //     let addr_sk_1 = get_addr_sk_1();

    //     let addr_pk_1 = hasher.mimc_single(&addr_sk_1).unwrap().to_bytes();

    //     let rho_1 = get_rho_1();

    //     let r_1 = get_r_1();

    //     let sn = hasher.mimc(&addr_sk_1, &rho_1).unwrap().to_bytes();

    //     sn
    // };

    // let sn_2 = {
    //     let addr_sk_1 = get_addr_sk_1();

    //     let addr_pk_1: U8Arr32 =
    //         hasher.mimc_single(&addr_sk_1).unwrap().to_bytes();

    //     let rho_2 = get_rho_1();

    //     let r_2 = get_r_1();

    //     let sn = hasher.mimc(&addr_sk_1, &rho_2).unwrap().to_bytes();

    //     sn
    // };

    // let pi = vec![0];

    // let pour_tc = PourTxCandidate::new(
    //     String::from("created_at_1"),
    //     WASM_MAGIC_NUMBER.to_vec(),
    //     String::from("author_sig_1"),
    //     Some(String::from("ctr_addr_1")),
    //     vec![11, 11, 11],
    //     U8Array::new_empty_32(),
    //     U8Array::new_empty_32(),
    //     U8Array::new_empty_32(),
    //     U8Array::new_empty_32(),
    // );

    // TxCandidate::Pour(pour_tc)

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

    // let proof_context = make_proof_context();

    let coin_1_old = OldCoin {
        addr_pk: Some(addr_pk_1_old),
        addr_sk: Some(addr_sk_1_old),
        rho: Some(rho_1_old),
        r: Some(r_1_old),
        s: Some(s_1_old),
        v: Some(v_1_old),
        cm: Some(cm_1_old),
        auth_path: auth_path_1,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(addr_pk_1),
        rho: Some(rho_1),
        r: Some(r_1),
        s: Some(s_1),
        v: Some(v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(addr_pk_2),
        rho: Some(rho_2),
        r: Some(r_2),
        s: Some(s_2),
        v: Some(v_2),
    };

    let pi =
        CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
            .unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi)?;

    let pour_tc = PourTxCandidate::new(
        "created_at".to_string(),
        vec![],
        "author_sig".to_string(),
        None,
        pi_serialized,
        sn_1.to_bytes(),
        cm_1.to_bytes(),
        cm_2.to_bytes(),
        merkle_rt.to_bytes(),
    );

    let c = TxCandidate::Pour(pour_tc);

    Ok(c)
}

pub fn mock_mint_tc_custom(
    cm: U8Arr32,
    v: U8Arr32,
    k: U8Arr32,
    s: U8Arr32,
) -> TxCandidate {
    let tx_candidate = MintTxCandidate::new_dummy_custom(cm, v, k, s);

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_1() -> TxCandidate {
    let tx_candidate = MintTxCandidate::new_dummy_1();

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_2() -> TxCandidate {
    let tx_candidate = MintTxCandidate::new_dummy_2();

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_3() -> TxCandidate {
    let tx_candidate = MintTxCandidate::new_dummy_3();

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_4() -> TxCandidate {
    let tx_candidate = MintTxCandidate::new_dummy_4();

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_deploying_contract(
    contract_data: Vec<u8>,
    ctrt_addr: String,
) -> TxCandidate {
    let tx_candidate =
        MintTxCandidate::new_dummy_deploying_contract(contract_data, ctrt_addr);

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_pour_tc_m1_to_p3_p4() -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_m1_to_p3_p4();

    TxCandidate::Pour(tx_candidate)
}

pub fn mock_pour_tc_2() -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_2();

    TxCandidate::Pour(tx_candidate)
}

pub fn mock_pour_tc_3() -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_3();

    TxCandidate::Pour(tx_candidate)
}

pub fn mock_pour_tc_4() -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_4();

    TxCandidate::Pour(tx_candidate)
}

pub fn new_dummy_valid_pour(
    pi: Vec<u8>,
    sn_1: [u8; 32],
    cm_1: [u8; 32],
    cm_2: [u8; 32],
    merkle_rt: [u8; 32],
) -> TxCandidate {
    let tx_candidate =
        PourTxCandidate::new_dummy_valid(pi, sn_1, cm_1, cm_2, merkle_rt);

    TxCandidate::Pour(tx_candidate)
}

pub fn mock_pour_tc_variant_cm(cm: [u8; 32]) -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_5(cm);

    TxCandidate::Pour(tx_candidate)
}

pub fn mock_pour_tc_variant_created_at() -> TxCandidate {
    let tx_candidate = PourTxCandidate::new_dummy_6();

    TxCandidate::Pour(tx_candidate)
}
