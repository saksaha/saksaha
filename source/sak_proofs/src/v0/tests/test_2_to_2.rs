use crate::CoinProof;
use crate::{ProofError, CM_TREE_DEPTH};
use bellman::groth16::{self, Parameters, Proof};
use sak_crypto::{Bls12, OsRng, Scalar, ScalarExt};
use sak_zkp_circuits::{
    CoinProofCircuit2to2, Hasher, MerkleTree, NewCoin, OldCoin,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use type_extension::U8Array;

const PARAM_FILE_NAME: &str = "mimc_params_2to2";

pub struct TestContext {
    pub hasher: Hasher,

    // old coin 1
    pub addr_pk_1_old: Scalar,
    pub addr_sk_1_old: Scalar,
    pub r_1_old: Scalar,
    pub s_1_old: Scalar,
    pub rho_1_old: Scalar,
    pub v_1_old: Scalar,
    pub cm_1_old: Scalar,
    pub auth_path_1: [(Scalar, bool); CM_TREE_DEPTH as usize],
    pub merkle_rt_1: Scalar,
    pub sn_1: Scalar,

    // old coin 2
    pub addr_pk_2_old: Scalar,
    pub addr_sk_2_old: Scalar,
    pub r_2_old: Scalar,
    pub s_2_old: Scalar,
    pub rho_2_old: Scalar,
    pub v_2_old: Scalar,
    pub cm_2_old: Scalar,
    pub auth_path_2: [(Scalar, bool); CM_TREE_DEPTH as usize],
    // pub merkle_rt_2: Scalar,
    pub sn_2: Scalar,

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

pub fn make_test_context_2_to_2() -> TestContext {
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
        addr_pk_2_old,
        addr_sk_2_old,
        r_2_old,
        s_2_old,
        rho_2_old,
        v_2_old,
        cm_2_old,
        sn_2,
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

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
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
            let arr = U8Array::from_int(80);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk = {
            let arr = U8Array::from_int(31);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(32);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(33);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(34);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(120);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

    let merkle_nodes_1 = mock_merkle_nodes_cm_1(&hasher, cm_1_old, cm_2_old);

    let merkle_rt_1 = *merkle_nodes_1
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);
        let mut ret = [(Scalar::default(), false); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!(
                    "Invalid assignment to a fixed sized array, idx: {}",
                    idx
                );
            }

            let key = format!("{}_{}", idx, p.idx);

            let merkle_node =
                merkle_nodes_1.get(key.as_str()).expect(&format!(
                    "value doesn't exist in the merkle node, key: {}",
                    key
                ));

            ret[idx] = (merkle_node.clone(), p.direction);
        });

        ret
    };

    let merkle_nodes_2 = mock_merkle_nodes_cm_2(&hasher, cm_1_old, cm_2_old);

    let merkle_rt_2 = *merkle_nodes_2
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    // println!("merkle_rt_1: {:?}", merkle_rt_1);
    // println!("merkle_rt_2: {:?}", merkle_rt_2);

    let auth_path_2 = {
        let v = merkle_tree.generate_auth_paths(1);
        let mut ret = [(Scalar::default(), false); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!(
                    "Invalid assignment to a fixed sized array, idx: {}",
                    idx
                );
            }

            let key = format!("{}_{}", idx, p.idx);

            let merkle_node =
                merkle_nodes_2.get(key.as_str()).expect(&format!(
                    "value doesn't exist in the merkle node, key: {}",
                    key
                ));

            ret[idx] = (merkle_node.clone(), p.direction);
        });

        ret
    };

    println!("auth_path_1: {:?}", auth_path_1);
    println!("auth_path_2: {:?}", auth_path_2);

    TestContext {
        hasher,
        addr_pk_1_old,
        addr_sk_1_old,
        r_1_old,
        s_1_old,
        rho_1_old,
        v_1_old,
        cm_1_old,
        auth_path_1,
        merkle_rt_1,
        sn_1,
        addr_pk_2_old,
        addr_sk_2_old,
        r_2_old,
        s_2_old,
        rho_2_old,
        v_2_old,
        cm_2_old,
        auth_path_2,
        // merkle_rt_2,
        sn_2,
        addr_sk_1,
        addr_pk_1,
        r_1,
        s_1,
        rho_1,
        v_1,
        cm_1,
        addr_sk_2,
        addr_pk_2,
        r_2,
        s_2,
        rho_2,
        v_2,
        cm_2,
    }
}

pub fn mock_merkle_nodes_cm_1(
    hasher: &Hasher,
    cm_old_1: Scalar,
    cm_old_2: Scalar,
) -> HashMap<&'static str, Scalar> {
    let merkle_nodes = {
        let mut m = HashMap::new();

        let node_0_1 = cm_old_2;

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

        let node_1_0 = hasher.mimc_scalar(cm_old_1, node_0_1);

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

    merkle_nodes
}

pub fn mock_merkle_nodes_cm_2(
    hasher: &Hasher,
    cm_old_1: Scalar,
    cm_old_2: Scalar,
) -> HashMap<&'static str, Scalar> {
    let merkle_nodes = {
        let mut m = HashMap::new();

        // let node_0_1 = {
        //     let arr = U8Array::new_empty_32();
        //     ScalarExt::parse_arr(&arr).unwrap()
        // };

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

        let node_1_0 = hasher.mimc_scalar(cm_old_1, cm_old_2);

        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);

        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);

        let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);

        m.insert("0_0", cm_old_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_1", node_3_1);
        m.insert("4_0", node_4_0);

        m
    };

    merkle_nodes
}

pub fn get_test_params_2_to_2(constants: &[Scalar]) -> Parameters<Bls12> {
    let param_path = std::path::Path::new(PARAM_FILE_NAME);
    let is_file_exist = param_path.exists();

    let mut v = vec![];

    if is_file_exist {
        // read
        v = std::fs::read(PARAM_FILE_NAME).unwrap();
    } else {
        // generate and write
        let hasher = Hasher::new();

        let coin_1_old = OldCoin::default();
        let coin_2_old = OldCoin::default();
        let coin_1_new = NewCoin::default();
        let coin_2_new = NewCoin::default();

        let params = {
            let c = CoinProofCircuit2to2 {
                hasher,
                coin_1_old,
                coin_2_old,
                coin_1_new,
                coin_2_new,
                constants: constants.to_vec(),
            };

            groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
                .unwrap()
        };
        // write param to file
        let mut file = File::create(PARAM_FILE_NAME).unwrap();

        params.write(&mut v).unwrap();
        // write origin buf
        file.write_all(&v).unwrap();
    }

    let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
    de_params
}

fn generate_proof_2_to_2(
    coin_1_old: OldCoin,
    coin_2_old: OldCoin,
    coin_1_new: NewCoin,
    coin_2_new: NewCoin,
) -> Result<Proof<Bls12>, ProofError> {
    let hasher = Hasher::new();

    let constants = hasher.get_mimc_constants().to_vec();
    let de_params = get_test_params_2_to_2(&constants);

    let c = CoinProofCircuit2to2 {
        hasher,
        coin_1_old,
        coin_2_old,
        coin_1_new,
        coin_2_new,
        constants,
    };

    let proof = match groth16::create_random_proof(c, &de_params, &mut OsRng) {
        Ok(p) => p,
        Err(err) => {
            return Err(format!(
                "Failed to generate groth16 proof, err: {}",
                err
            )
            .into());
        }
    };

    Ok(proof)
}

fn verify_proof_2_to_2(
    proof: Proof<Bls12>,
    public_inputs: &[Scalar],
    hasher: &Hasher,
) -> bool {
    let constants = hasher.get_mimc_constants();
    let de_params = get_test_params_2_to_2(&constants);
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

    println!("[+] proof: {:?}", proof);
    println!("[+] public input: {:?}", public_inputs);

    match groth16::verify_proof(&pvk, &proof, public_inputs) {
        Ok(_) => {
            println!("verify success!");
            true
        }
        Err(err) => {
            println!("verify_proof(), err: {}", err);
            false
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default_2_to_2() {
    sak_test_utils::init_test_log();

    let test_context = make_test_context_2_to_2();

    let coin_1_old = OldCoin {
        addr_pk: Some(test_context.addr_pk_1_old),
        addr_sk: Some(test_context.addr_sk_1_old),
        rho: Some(test_context.rho_1_old),
        r: Some(test_context.r_1_old),
        s: Some(test_context.s_1_old),
        v: Some(test_context.v_1_old),
        cm: Some(test_context.cm_1_old),
        auth_path: test_context.auth_path_1.map(|e| Some(e)),
    };

    let coin_2_old = OldCoin {
        addr_pk: Some(test_context.addr_pk_2_old),
        addr_sk: Some(test_context.addr_sk_2_old),
        rho: Some(test_context.rho_2_old),
        r: Some(test_context.r_2_old),
        s: Some(test_context.s_2_old),
        v: Some(test_context.v_2_old),
        cm: Some(test_context.cm_2_old),
        auth_path: test_context.auth_path_2.map(|e| Some(e)),
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(test_context.addr_pk_1),
        rho: Some(test_context.rho_1),
        r: Some(test_context.r_1),
        s: Some(test_context.s_1),
        v: Some(test_context.v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(test_context.addr_pk_2),
        rho: Some(test_context.rho_2),
        r: Some(test_context.r_2),
        s: Some(test_context.s_2),
        v: Some(test_context.v_2),
    };

    let proof = CoinProof::generate_proof_2_to_2(
        coin_1_old, coin_2_old, coin_1_new, coin_2_new,
    )
    .expect("proof should be created");

    let public_inputs: Vec<Scalar> = vec![
        test_context.merkle_rt_1,
        test_context.sn_1,
        test_context.sn_2,
        test_context.cm_1,
        test_context.cm_2,
    ];

    assert_eq!(
        CoinProof::verify_proof_2_to_2(
            proof,
            &public_inputs,
            &test_context.hasher
        )
        .unwrap(),
        true
    );
}
