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
use std::fs::File;
use std::io::Write;

const TEST_TREE_DEPTH: usize = 3;

fn make_test_context() -> (
    // merkle root
    Scalar,
    // old coin 1
    Scalar, // addr_sk_1_old,
    Scalar, // r_1_old,
    Scalar, // s_1_old,
    Scalar, // rho_1_old,
    Scalar, // v_1_old,
    // new coin 1
    Scalar, // addr_sk_1,
    Scalar, // r_1,
    Scalar, // s_1,
    Scalar, // rho_1,
    Scalar, // v_1,
    // new coin 2
    Scalar, // addr_sk_2,
    Scalar, // r_2,
    Scalar, // s_2,
    Scalar, // rho_2,
    Scalar, // v_2,
) {
    let hasher = Hasher::new();

    let (addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old) = {
        let addr_sk = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let r = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(100);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        (addr_sk, r, s, rho, v)
    };

    let (addr_sk_1, r_1, s_1, rho_1, v_1) = {
        let addr_sk = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let r = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(100);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        (addr_sk, r, s, rho, v)
    };

    let (addr_sk_2, r_2, s_2, rho_2, v_2) = {
        let addr_sk = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let r = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::new_empty_32();
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(100);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        (addr_sk, r, s, rho, v)
    };

    let constants = mimc::get_mimc_constants();

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];

    let merkle_tree = MerkleTree::new(3, &constants);

    let merkle_rt = {
        let arr = U8Array::new_empty_32();
        ScalarExt::parse_arr(&arr).unwrap()
    };

    (
        merkle_rt, //
        // old coin 1
        addr_sk_1_old, // [gen_proof] : secret key
        r_1_old,       // [gen_proof] : random sample value `r`
        s_1_old,       // [gen_proof] : random sample value `s`
        rho_1_old,     // [gen_proof] : rho value
        v_1_old,       // [gen_proof] : value of coin `v`
        // new coin 1
        addr_sk_1,
        r_1,
        s_1,
        rho_1,
        v_1,
        // new coin 2
        addr_sk_2,
        r_2,
        s_2,
        rho_2,
        v_2,
    )
}

pub fn get_params_test(constants: &[Scalar]) -> Parameters<Bls12> {
    let is_file_exist = std::path::Path::new("mimc_params").exists();
    let mut v = vec![];
    if is_file_exist {
        // read
        v = std::fs::read("mimc_params").unwrap();
    } else {
        // generate and write
        let hasher = Hasher::new();

        let params = {
            let c = CoinProofCircuit1to2 {
                hasher,

                // old coins
                addr_sk_1_old: None,
                rho_1_old: None,
                r_1_old: None,
                s_1_old: None,
                v_1_old: None,
                auth_path_1: [None; TEST_TREE_DEPTH],
                merkle_rt: None,

                // new coin 1
                addr_sk_1: None,
                rho_1: None,
                r_1: None,
                s_1: None,
                v_1: None,

                // new coin 2
                addr_sk_2: None,
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
        let mut file = File::create("mimc_params").unwrap();

        params.write(&mut v).unwrap();
        // write origin buf
        file.write_all(&v);
    }

    let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
    de_params
}

fn make_proof(
    tgt_leaf_idx: usize,

    // old coins
    addr_sk_1_old: Scalar,
    rho_1_old: Scalar,
    r_1_old: Scalar,
    s_1_old: Scalar,
    v_1_old: Scalar,
    auth_path_1: [(Scalar, bool); 3],
    merkle_rt: Scalar,

    // new coin 1
    addr_sk_1: Scalar,
    rho_1: Scalar,
    r_1: Scalar,
    s_1: Scalar,
    v_1: Scalar,

    // new coin 1
    addr_sk_2: Scalar,
    rho_2: Scalar,
    r_2: Scalar,
    s_2: Scalar,
    v_2: Scalar,
) -> Result<Proof<Bls12>, ProofError> {
    let constants = mimc::get_mimc_constants();
    let de_params = get_params_test(&constants);

    let addr_sk_1_old = Some(addr_sk_1_old);
    let rho_1_old = Some(rho_1_old);
    let r_1_old = Some(r_1_old);
    let s_1_old = Some(s_1_old);
    let v_1_old = Some(v_1_old);

    let auth_path_1 = auth_path_1.map(|p| Some(p));
    let merkle_rt = Some(merkle_rt);

    let addr_sk_1 = Some(addr_sk_1);
    let rho_1 = Some(rho_1);
    let r_1 = Some(r_1);
    let s_1 = Some(s_1);
    let v_1 = Some(v_1);
    //
    let addr_sk_2 = Some(addr_sk_2);
    let rho_2 = Some(rho_2);
    let r_2 = Some(r_2);
    let s_2 = Some(s_2);
    let v_2 = Some(v_2);

    let hasher = Hasher::new();

    let c = CoinProofCircuit1to2 {
        hasher,
        //
        addr_sk_1_old,
        rho_1_old,
        r_1_old,
        s_1_old,
        v_1_old,

        //
        auth_path_1,
        merkle_rt,
        //
        addr_sk_1,
        rho_1,
        r_1,
        s_1,
        v_1,
        //
        addr_sk_2,
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

    // println!("[+] proof: {:?}", proof);

    Ok(proof)
}

fn verify_proof(proof: Proof<Bls12>, public_inputs: &[Scalar]) -> bool {
    let constants = mimc::get_mimc_constants();
    let de_params = get_params_test(&constants);

    // Prepare the verification key (for proof verification).
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

    println!("[public_inputs] rt: {:?}", public_inputs[0]);
    println!("[public_inputs] pk: {:?}", public_inputs[1]);
    println!("[public_inputs] sn: {:?}", public_inputs[2]);
    println!("[public_inputs] k:  {:?}", public_inputs[3]);
    println!("[public_inputs] cm: {:?}", public_inputs[4]);

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
    // sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    sak_test_utils::init_test_log();

    println!("[!] test coin ownership start!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    println!("\n[+] Test Context creating");

    let (
        merkle_rt, // merkle tree
        //
        addr_sk_1_old, // secret key
        r_1_old,       // random sample value `r`
        s_1_old,       // random sample value `s`
        rho_1_old,     // rho value
        v_1_old,       // value of coin `v`
        //
        addr_sk_1, // secret key
        r_1,       // random sample value `r`
        s_1,       // random sample value `s`
        rho_1,     // rho value
        v_1,       // value of coin `v`
        //
        addr_sk_2, // secret key
        r_2,       // random sample value `r`
        s_2,       // random sample value `s`
        rho_2,     // rho value
        v_2,       // value of coin `v`
    ) = make_test_context();

    // let rt = mt.get_root().hash; // root hash value

    let tgt_leaf_idx = 0;

    println!("\n[+] Test Proof calculating");

    let proof = make_proof(
        tgt_leaf_idx,
        addr_sk_1_old,
        rho_1,
        r_1,
        s_1,
        v_1,
        merkle_rt,
        //
        addr_sk_1,
        rho_1,
        r_1,
        s_1,
        v_1,
        //
        addr_sk_2,
        rho_2,
        r_2,
        s_2,
        v_2,
    )
    .unwrap();

    // println!("\n[+] Test Verificationn");
    // let public_inputs: Vec<Scalar> = vec![
    //     rt, //
    //     a_pk_1, sn_1, k_1, cm_1, //
    //     a_pk_2, sn_2, k_2, cm_2,
    // ];
    // let result = verify_proof(proof, &public_inputs);

    // assert!(result);
}
