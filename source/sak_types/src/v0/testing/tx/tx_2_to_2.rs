use crate::mock_coin_custom;
use crate::{PourTxCandidate, TxCandidate};
use sak_crypto::Scalar;
use sak_crypto::{MerkleTreeSim, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_proof::CoinProof;
use sak_proof::NewCoin;
use sak_proof::OldCoin;

pub fn mock_pour_tc_2to2_1() -> TxCandidate {
    let old_coin_1 = mock_coin_custom(0x1, 0x2, 0x3, 0x4, 1000);
    let old_coin_2 = mock_coin_custom(0, 0, 0, 0, 1000); // dummy coin

    let new_coin_1 = mock_coin_custom(0x21, 0x22, 0x23, 0x24, 1990);
    let new_coin_2 = mock_coin_custom(0x31, 0x32, 0x33, 0x34, 0);

    let cm_1 = ScalarExt::parse_arr(&old_coin_1.cm).unwrap();
    let cm_2 = ScalarExt::parse_arr(&old_coin_2.cm).unwrap();

    let tree_simulator = MerkleTreeSim::init(CM_TREE_DEPTH as u32, vec![cm_1, cm_2]).unwrap();

    let merkle_tree = tree_simulator.merkle_tree;

    let merkle_nodes = tree_simulator.nodes;

    let merkle_rt_1 = *merkle_nodes
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();
    let merkle_rt_2 = *merkle_nodes
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);

        let mut ret = [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    let auth_path_2 = {
        let v = merkle_tree.generate_auth_paths(1);

        let mut ret = [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

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
        auth_path: auth_path_2,
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

    let pi =
        CoinProof::generate_proof_2_to_2(coin_1_old, coin_2_old, coin_1_new, coin_2_new).unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi).unwrap();

    println!(
        "************* coin_1_old sn:{:?}",
        coin_1_old.compute_sn().unwrap().to_bytes()
    );

    println!(
        "************* coin_2_old sn:{:?}",
        coin_2_old.compute_sn().unwrap().to_bytes()
    );

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
        vec![new_coin_1.cm, new_coin_2.cm],
        vec![merkle_rt_1.to_bytes(), merkle_rt_2.to_bytes()],
    );

    let c = TxCandidate::Pour(pour_tc);

    c
}
