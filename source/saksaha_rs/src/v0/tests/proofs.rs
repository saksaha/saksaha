use sak_crypto::hasher::MiMC;
use sak_crypto::Scalar;
use sak_crypto::{MerkleTreeSim, ScalarExt};
use sak_logger::SakLogger;
use sak_proof::{CoinProof, NewCoin, OldCoin};

use sak_dist_ledger_meta::CM_TREE_DEPTH;
use type_extension::U8Array;

fn make_test_context() -> (OldCoin, NewCoin, NewCoin, Scalar) {
    let hasher = MiMC::new();

    let (addr_pk_1_old, addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old, cm_1_old, sn_1) = {
        let addr_sk = {
            let arr = U8Array::from_int(1);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(2);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(3);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(4);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(100);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

    let (_addr_sk_1_new, addr_pk_1_new, r_1_new, s_1_new, rho_1_new, v_1_new, _cm_1_new) = {
        let addr_sk = {
            let arr = U8Array::from_int(11);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(12);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(13);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(14);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(60);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (_addr_sk_2_new, addr_pk_2_new, r_2_new, s_2_new, rho_2_new, v_2_new, _cm_2_new) = {
        let addr_sk = {
            let arr = U8Array::from_int(21);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(22);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(23);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(24);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(40);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let tree_simulator = MerkleTreeSim::init(CM_TREE_DEPTH as u32, vec![cm_1_old]).unwrap();

    let merkle_tree = tree_simulator.merkle_tree;

    let merkle_nodes = tree_simulator.nodes;

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);

        let mut ret = [(Scalar::default(), false); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = merkle_nodes.get(key.as_str()).unwrap();

            ret[idx] = (merkle_node.clone(), p.direction);
        });

        ret
    };

    (
        OldCoin {
            addr_pk: Some(addr_pk_1_old),
            addr_sk: Some(addr_sk_1_old),
            rho: Some(rho_1_old),
            r: Some(r_1_old),
            s: Some(s_1_old),
            v: Some(v_1_old),
            cm: Some(cm_1_old),
            auth_path: auth_path_1.map(|p| Some(p)),
        },
        NewCoin {
            addr_pk: Some(addr_pk_1_new),
            rho: Some(rho_1_new),
            r: Some(r_1_new),
            s: Some(s_1_new),
            v: Some(v_1_new),
        },
        NewCoin {
            addr_pk: Some(addr_pk_2_new),
            rho: Some(rho_2_new),
            r: Some(r_2_new),
            s: Some(s_2_new),
            v: Some(v_2_new),
        },
        merkle_nodes
            .get(format!("{}_0", CM_TREE_DEPTH).as_str())
            .unwrap()
            .to_owned(),
    )
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_make_a_proof_1_to_2_and_verify_it() {
    SakLogger::init_test_console().unwrap();

    let (coin_1_old, coin_1_new, coin_2_new, _merkle_rt) = make_test_context();

    let proof = CoinProof::generate_proof_1_to_2(
        //
        coin_1_old, //
        coin_1_new, //
        coin_2_new,
    )
    .unwrap();

    println!("proof: {:#?}", proof);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_auth_path() {
    SakLogger::init_test_console().unwrap();

    let saksaha_endpoint = "http://localhost:34418/rpc/v0".to_string();
    let idx: u128 = 0;
    let resp = crate::get_auth_path(saksaha_endpoint, idx).await.unwrap();

    let auth_path = resp.result.unwrap();
    println!("[+] auth_path: {:?}", auth_path);
}
