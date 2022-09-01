use super::TestUtil;
use sak_crypto::{Bls12, Scalar, ScalarExt};
use sak_proofs::{
    CoinProof, Hasher, MerkleTree, NewCoin, OldCoin, Proof, CM_TREE_DEPTH,
};

#[tokio::test(flavor = "multi_thread")]
async fn test_generate_a_proof() {
    sak_test_utils::init_test_log();

    TestUtil::init_test(vec!["test"]);

    let coin_1_old = sak_types::mock_coin(100);

    println!("coin: {}", coin_1_old);

    let tx_candidates = vec![
        //
        sak_types::mock_mint_tc_custom(
            coin_1_old.cm,
            coin_1_old.v,
            coin_1_old.k,
            coin_1_old.s,
        ),
    ];

    let genesis_block = sak_types::mock_block(tx_candidates);

    let dist_ledger = sak_dist_ledger::mock_dist_ledger(genesis_block).await;

    let cm_1_old_idx: u128 = 0;

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);
    let auth_path_idx = merkle_tree.generate_auth_paths(cm_1_old_idx);

    let mut auth_path =
        [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

    println!("[*] initial auth_path: {:#?}", auth_path);

    for (idx, p) in auth_path_idx.iter().enumerate() {
        if idx >= auth_path.len() {
            panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
        }

        println!("auth_path: {}_{}", idx, p.idx);
        let key = format!("{}_{}", idx, p.idx);

        let merkle_node = dist_ledger.apis.get_merkle_node(&key).await.unwrap();

        let merkle_node = ScalarExt::parse_arr(&merkle_node).unwrap();

        auth_path[idx] = Some((merkle_node, p.direction));
    }

    println!("[*] updated auth_path: {:#?}", auth_path);

    let coin_1_new = sak_types::mock_coin(60);
    println!("coin: {}", coin_1_new);

    let coin_2_new = sak_types::mock_coin(40);
    println!("coin: {}", coin_2_new);

    let coin_1_old = OldCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_1_old.addr_pk).unwrap()),
        addr_sk: Some(ScalarExt::parse_arr(&coin_1_old.addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_1_old.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_1_old.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_1_old.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_1_old.v).unwrap()),
        cm: Some(ScalarExt::parse_arr(&coin_1_old.cm).unwrap()),
        auth_path,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_1_new.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_1_new.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_1_new.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_1_new.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_1_new.v).unwrap()),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_2_new.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_2_new.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_2_new.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_2_new.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_2_new.v).unwrap()),
    };

    println!("\n[+] Waiting for generating pi...");

    let pi =
        CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
            .unwrap();

    println!("[!] pi: {:#?}", pi);

    {
        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized, len: {}, {:?}", pi_ser.len(), pi_ser);

        let pi_des: Proof<Bls12> = Proof::read(&*pi_ser).unwrap();

        println!("[!] pi deserialized: {:#?}", pi_des);
    }

    println!("\n[+] Verifying  pi...");

    {
        let hasher = Hasher::new();

        let merkle_rt = {
            let mut merkle_rt = coin_1_old.cm.unwrap();

            for i in auth_path {
                let (v, _) = i.unwrap();

                merkle_rt = hasher.mimc_scalar(merkle_rt, v);
            }

            merkle_rt
        };

        let sn_1_old = hasher.mimc_scalar(
            coin_1_old.clone().addr_sk.unwrap(),
            coin_1_old.clone().rho.unwrap(),
        );

        let cm_1_new = {
            let k = hasher.comm2_scalar(
                coin_1_new.clone().r.unwrap(),
                coin_1_new.clone().addr_pk.unwrap(),
                coin_1_new.clone().rho.unwrap(),
            );

            let cm_1_new = hasher.comm2_scalar(
                coin_1_new.clone().s.unwrap(),
                coin_1_new.clone().v.unwrap(),
                k,
            );

            cm_1_new
        };

        let cm_2_new = {
            let k = hasher.comm2_scalar(
                coin_2_new.clone().r.unwrap(),
                coin_2_new.clone().addr_pk.unwrap(),
                coin_2_new.clone().rho.unwrap(),
            );

            let cm_2_new = hasher.comm2_scalar(
                coin_2_new.clone().s.unwrap(),
                coin_2_new.clone().v.unwrap(),
                k,
            );

            cm_2_new
        };

        let public_inputs = [merkle_rt, sn_1_old, cm_1_new, cm_2_new];

        assert_eq!(
            CoinProof::verify_proof_1_to_2(pi, &public_inputs, &hasher)
                .unwrap(),
            true
        );
    }
}
