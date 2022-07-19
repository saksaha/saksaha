use crate::{CoinProofCircuit1to2, MerkleTree, Path, ProofError};
use rand::rngs::OsRng;
use rand::RngCore;
use sak_crypto::{
    groth16, AllocatedBit, Circuit, ConstraintSystem, Proof, ScalarExt,
    SynthesisError,
};
use sak_crypto::{mimc, Parameters};
use sak_crypto::{Bls12, Hasher, Scalar};
use sak_types::U8Array;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const TEST_TREE_DEPTH: usize = 3;
const PARAM_FILE_NAME: &str = "mimc_params";

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
    pub auth_path_1: [(Scalar, bool); 3],
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

fn make_test_context() -> TestContext {
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

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
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

            let ret = hasher.comm2_scalar(s, v, k);

            // println!("make test context cm_1!!!123123: {:?}",);
            // println!("make test context cm_1!!!123123: {:?}", ret);
            // println!("make test context cm_1!!!123123: {:?}", ret);

            ret
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
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

    let constants = hasher.get_mimc_constants();

    let merkle_tree = MerkleTree::new(TEST_TREE_DEPTH as u32, &constants);

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

        let node_1_0 = hasher.mimc_scalar(cm_1_old, node_0_1);

        let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);

        let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);

        m.insert("0_1", node_0_1);
        m.insert("1_1", node_1_1);
        m.insert("2_1", node_2_1);
        m.insert("3_0", node_3_0);

        m
    };

    let merkle_rt = *merkle_nodes.get("3_0").unwrap();

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);
        let mut ret = [(Scalar::default(), false); TEST_TREE_DEPTH];

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
        merkle_rt,
        sn_1,
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

pub fn get_test_params(constants: &[Scalar]) -> Parameters<Bls12> {
    let param_path = std::path::Path::new(PARAM_FILE_NAME);
    let is_file_exist = param_path.exists();
    println!("param file exists!!, path: {:?}", param_path);

    let mut v = vec![];

    if is_file_exist {
        // read
        v = std::fs::read(PARAM_FILE_NAME).unwrap();
    } else {
        // generate and write
        let hasher = Hasher::new();

        let params = {
            let c = CoinProofCircuit1to2 {
                hasher,

                // old coins
                addr_pk_1_old: None,
                addr_sk_1_old: None,
                rho_1_old: None,
                r_1_old: None,
                s_1_old: None,
                v_1_old: None,
                cm_1_old: None,
                auth_path_1: [None; TEST_TREE_DEPTH],

                // new coin 1
                addr_pk_1: None,
                rho_1: None,
                r_1: None,
                s_1: None,
                v_1: None,

                // new coin 2
                addr_pk_2: None,
                rho_2: None,
                r_2: None,
                s_2: None,
                v_2: None,
                constants: constants.to_vec(),
            };

            groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
                .unwrap()
        };
        // write param to file
        let mut file = File::create(PARAM_FILE_NAME).unwrap();

        params.write(&mut v).unwrap();
        // write origin buf
        file.write_all(&v);
    }

    let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
    de_params
}

fn make_proof(
    // old coins
    addr_pk_1_old: Scalar,
    addr_sk_1_old: Scalar,
    rho_1_old: Scalar,
    r_1_old: Scalar,
    s_1_old: Scalar,
    v_1_old: Scalar,
    cm_1_old: Scalar,
    auth_path_1: [(Scalar, bool); 3],

    // new coin 1
    addr_pk_1: Scalar,
    rho_1: Scalar,
    r_1: Scalar,
    s_1: Scalar,
    v_1: Scalar,

    // new coin 1
    addr_pk_2: Scalar,
    rho_2: Scalar,
    r_2: Scalar,
    s_2: Scalar,
    v_2: Scalar,
) -> Result<Proof<Bls12>, ProofError> {
    println!("power!!! auth path: {:#?}", auth_path_1);

    let hasher = Hasher::new();

    let constants = hasher.get_mimc_constants().to_vec();
    let de_params = get_test_params(&constants);

    let addr_pk_1_old = Some(addr_pk_1_old);
    let addr_sk_1_old = Some(addr_sk_1_old);
    let rho_1_old = Some(rho_1_old);
    let r_1_old = Some(r_1_old);
    let s_1_old = Some(s_1_old);
    let v_1_old = Some(v_1_old);
    let cm_1_old = Some(cm_1_old);
    let auth_path_1 = auth_path_1.map(|p| Some(p));

    //
    let addr_pk_1 = Some(addr_pk_1);
    let rho_1 = Some(rho_1);
    let r_1 = Some(r_1);
    let s_1 = Some(s_1);
    let v_1 = Some(v_1);

    //
    let addr_pk_2 = Some(addr_pk_2);
    let rho_2 = Some(rho_2);
    let r_2 = Some(r_2);
    let s_2 = Some(s_2);
    let v_2 = Some(v_2);

    let c = CoinProofCircuit1to2 {
        hasher,

        // old coin 1
        addr_pk_1_old,
        addr_sk_1_old,
        rho_1_old,
        r_1_old,
        s_1_old,
        v_1_old,
        cm_1_old,
        auth_path_1,

        // new coin 1
        addr_pk_1,
        rho_1,
        r_1,
        s_1,
        v_1,

        // new coin 2
        addr_pk_2,
        rho_2,
        r_2,
        s_2,
        v_2,
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

    println!("[+] proof: {:?}", proof);

    Ok(proof)
}

fn verify_proof(
    proof: Proof<Bls12>,
    public_inputs: &[Scalar],
    hasher: &Hasher,
) -> bool {
    let constants = hasher.get_mimc_constants();
    let de_params = get_test_params(&constants);
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

    println!("[public_inputs] {:?}", public_inputs);

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
pub async fn test_coin_ownership_default() {
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    sak_test_utils::init_test_log();

    let test_context = make_test_context();

    let proof = make_proof(
        test_context.addr_pk_1_old,
        test_context.addr_sk_1_old,
        test_context.rho_1_old,
        test_context.r_1_old,
        test_context.s_1_old,
        test_context.v_1_old,
        test_context.cm_1_old,
        test_context.auth_path_1,
        //
        // test_context.addr_sk_1,
        test_context.addr_pk_1,
        test_context.rho_1,
        test_context.r_1,
        test_context.s_1,
        test_context.v_1,
        //
        // test_context.addr_sk_2,
        test_context.addr_pk_2,
        test_context.rho_2,
        test_context.r_2,
        test_context.s_2,
        test_context.v_2,
    )
    .unwrap();

    println!("proof generated!!");

    let public_inputs: Vec<Scalar> = vec![
        test_context.merkle_rt,
        test_context.sn_1,
        test_context.cm_1,
        test_context.cm_2,
    ];

    let result = verify_proof(proof, &public_inputs, &test_context.hasher);

    assert!(result);
}
