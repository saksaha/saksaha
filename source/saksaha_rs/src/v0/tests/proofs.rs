use sak_crypto::{rand, ScalarExt};
use sak_crypto::{Hasher, Scalar};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use std::collections::HashMap;
use type_extension::U8Array;

use crate::{generate_proof_1_to_2, get_auth_path};

fn make_test_context() -> (OldCoin, NewCoin, NewCoin, Scalar) {
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

    let (
        addr_sk_1_new,
        addr_pk_1_new,
        r_1_new,
        s_1_new,
        rho_1_new,
        v_1_new,
        cm_1_new,
    ) = {
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

    let (
        addr_sk_2_new,
        addr_pk_2_new,
        r_2_new,
        s_2_new,
        rho_2_new,
        v_2_new,
        cm_2_new,
    ) = {
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

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH);

    let merkle_nodes = {
        let mut m = HashMap::new();

        let node_0_1 = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let node_1_1 = {
            let node_0_2 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_3 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let h = hasher.mimc_scalar(node_0_2, node_0_3);
            h
        };

        let node_2_1 = {
            let node_0_4 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_5 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_6 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_7 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_1_2 = hasher.mimc_scalar(node_0_4, node_0_5);

            let node_1_3 = hasher.mimc_scalar(node_0_6, node_0_7);

            hasher.mimc_scalar(node_1_2, node_1_3)
        };

        let node_3_1 = {
            let node_0_8 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_9 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_10 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_11 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_12 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_13 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_14 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            let node_0_15 = {
                let arr = U8Array::new_empty_32();
                ScalarExt::parse_arr(&arr).unwrap()
            };

            //
            let node_1_4 = hasher.mimc_scalar(node_0_8, node_0_9);

            let node_1_5 = hasher.mimc_scalar(node_0_10, node_0_11);

            let node_1_6 = hasher.mimc_scalar(node_0_12, node_0_13);

            let node_1_7 = hasher.mimc_scalar(node_0_14, node_0_15);

            //
            let node_2_2 = hasher.mimc_scalar(node_1_4, node_1_5);

            let node_2_3 = hasher.mimc_scalar(node_1_6, node_1_7);

            hasher.mimc_scalar(node_2_2, node_2_3)
        };

        let node_1_0 = hasher.mimc_scalar(cm_1_old, node_0_1);

        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);

        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);

        let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);

        m.insert("0_1", node_0_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_1", node_3_1);
        m.insert("4_0", node_4_0);

        m
    };

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);

        let mut ret = [(Scalar::default(), false); 4 as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!(
                    "Invalid assignment to a fixed sized array, idx: {}",
                    idx
                );
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
        merkle_nodes.get("4_0").unwrap().to_owned(),
    )
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_make_a_proof_and_verify_it() {
    sak_test_utils::init_test_log();

    let (coin_1_old, coin_1_new, coin_2_new, merkle_rt) = make_test_context();

    let proof = generate_proof_1_to_2(
        //
        coin_1_old, //
        coin_1_new, //
        coin_2_new,
    )
    .await;

    println!("proof: {:#?}", proof);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_auth_path() {
    sak_test_utils::init_test_log();

    let idx: u128 = 0;
    let resp = get_auth_path(idx).await.unwrap();

    let auth_path = resp.result.unwrap();
    println!("[+] auth_path: {:?}", auth_path);
}
