use crate::v0::testing::values;
use crate::{get_addr_sk_1, MockCoin};
use crate::{
    mock_coin_custom, Cm, MintTxCandidate, PourTxCandidate, Sn, Tx, VALIDATOR,
    VALIDATOR_CTR_ADDR,
};
use crate::{TxCandidate, TypesError};
use sak_crypto::MerkleTree;
use sak_crypto::{rand, Scalar};
use sak_crypto::{MerkleTreeSim, ScalarExt};
use sak_dist_ledger_meta::CM_TREE_DEPTH;
use sak_proofs::CoinProof;
use sak_proofs::Hasher;
use sak_proofs::NewCoin;
use sak_proofs::OldCoin;
use std::collections::HashMap;
use std::io::Read;
use type_extension::U8Array;

pub fn mock_pour_tc_2to2_1() -> TxCandidate {
    let hasher = Hasher::new();

    let old_coin_1 = mock_coin_custom(0x1, 0x2, 0x3, 0x4, 1000);

    let old_coin_2 = mock_coin_custom(0, 0, 0, 0, 1000); // dummy coin

    let new_coin_1 = mock_coin_custom(0x21, 0x22, 0x23, 0x24, 1990);
    let new_coin_2 = mock_coin_custom(0x31, 0x32, 0x33, 0x34, 0);

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

    let merkle_nodes_1 = {
        let cm = ScalarExt::parse_arr(&old_coin_1.cm).unwrap();

        let mut m = HashMap::new();

        let node_0_1 = ScalarExt::parse_u64(0).unwrap();
        let node_1_1 = ScalarExt::parse_u64(0).unwrap();
        let node_2_1 = ScalarExt::parse_u64(0).unwrap();
        let node_3_1 = ScalarExt::parse_u64(0).unwrap();
        let node_4_1 = ScalarExt::parse_u64(0).unwrap();
        let node_5_1 = ScalarExt::parse_u64(0).unwrap();

        m.insert("0_1", node_0_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_1", node_3_1);
        m.insert("4_1", node_4_1);
        m.insert("5_1", node_5_1);

        let node_1_0 = hasher.mimc_scalar(cm, node_0_1);
        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);
        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);
        let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);
        let node_5_0 = hasher.mimc_scalar(node_4_0, node_4_1);
        let node_6_0 = hasher.mimc_scalar(node_5_0, node_5_1);

        m.insert("1_0", node_1_0);
        m.insert("2_0", node_2_0);
        m.insert("3_0", node_3_0);
        m.insert("4_0", node_4_0);
        m.insert("5_0", node_5_0);
        m.insert("6_0", node_6_0);

        m
    };

    let merkle_nodes_2 = {
        let cm = ScalarExt::parse_arr(&old_coin_2.cm).unwrap();

        let mut m = HashMap::new();

        let node_0_1 = ScalarExt::parse_u64(0).unwrap();
        let node_1_1 = ScalarExt::parse_u64(0).unwrap();
        let node_2_1 = ScalarExt::parse_u64(0).unwrap();
        let node_3_1 = ScalarExt::parse_u64(0).unwrap();
        let node_4_1 = ScalarExt::parse_u64(0).unwrap();
        let node_5_1 = ScalarExt::parse_u64(0).unwrap();

        m.insert("0_1", node_0_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_1", node_3_1);
        m.insert("4_1", node_4_1);
        m.insert("5_1", node_5_1);

        let node_1_0 = hasher.mimc_scalar(cm, node_0_1);
        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);
        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);
        let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);
        let node_5_0 = hasher.mimc_scalar(node_4_0, node_4_1);
        let node_6_0 = hasher.mimc_scalar(node_5_0, node_5_1);

        m.insert("1_0", node_1_0);
        m.insert("2_0", node_2_0);
        m.insert("3_0", node_3_0);
        m.insert("4_0", node_4_0);
        m.insert("5_0", node_5_0);
        m.insert("6_0", node_6_0);

        m
    };

    let merkle_rt_1 = *merkle_nodes_1.get("6_0").unwrap();
    let merkle_rt_2 = *merkle_nodes_2.get("6_0").unwrap();

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

            let empty_node = ScalarExt::parse_u64(0).unwrap();

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node =
                merkle_nodes_1.get(key.as_str()).unwrap_or(&empty_node);

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    let auth_path_2 = {
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

            let empty_node = ScalarExt::parse_u64(0).unwrap();

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node =
                merkle_nodes_2.get(key.as_str()).unwrap_or(&empty_node);

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    let coin_1_old = OldCoin {
        addr_pk: Some(ScalarExt::parse_arr(&old_coin_1.addr_pk).unwrap()),
        addr_sk: Some(ScalarExt::parse_arr(&old_coin_1.addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&old_coin_1.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&old_coin_1.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&old_coin_1.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&old_coin_1.v).unwrap()),
        cm: Some(ScalarExt::parse_arr(&old_coin_1.cm).unwrap()),
        auth_path: auth_path_1,
    };

    let coin_2_old = OldCoin {
        addr_pk: Some(ScalarExt::parse_arr(&old_coin_2.addr_pk).unwrap()),
        addr_sk: Some(ScalarExt::parse_arr(&old_coin_2.addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&old_coin_2.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&old_coin_2.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&old_coin_2.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&old_coin_2.v).unwrap()),
        cm: Some(ScalarExt::parse_arr(&old_coin_2.cm).unwrap()),
        auth_path: auth_path_1,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&new_coin_1.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&new_coin_1.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&new_coin_1.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&new_coin_1.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&new_coin_1.v).unwrap()),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&new_coin_2.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&new_coin_2.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&new_coin_2.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&new_coin_2.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&new_coin_2.v).unwrap()),
    };

    // let pi =
    //     CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
    //         .unwrap();

    let pi = CoinProof::generate_proof_2_to_2(
        coin_1_old, coin_2_old, coin_1_new, coin_2_new,
    )
    .unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi).unwrap();

    let pour_tc = PourTxCandidate::new(
        "created_at".to_string(),
        vec![],
        "author_sig".to_string(),
        None,
        pi_serialized,
        vec![
            coin_1_old.compute_sn().unwrap().to_bytes(),
            coin_2_old.compute_sn().unwrap().to_bytes(),
        ],
        vec![
            coin_1_old.cm.unwrap().to_bytes(),
            coin_2_old.cm.unwrap().to_bytes(),
        ],
        vec![merkle_rt_1.to_bytes(), merkle_rt_2.to_bytes()],
    );

    let c = TxCandidate::Pour(pour_tc);

    c
}
