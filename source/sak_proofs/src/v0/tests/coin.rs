use crate::{MerkleTree, Path, ProofError};
use rand::rngs::OsRng;
use rand::RngCore;
use sak_crypto::{
    groth16, AllocatedBit, Circuit, ConstraintSystem, Proof, SynthesisError,
};
use sak_crypto::{mimc, Parameters};
use sak_crypto::{Bls12, Hasher, Scalar};
use std::fs::File;
use std::io::Write;

const TEST_TREE_DEPTH: usize = 3;

struct TestCoinCircuit {
    pub hasher: Hasher,

    // old coins
    pub cm_old_1: Option<[u8; 32]>,
    pub cm_old_2: Option<[u8; 32]>,
    pub auth_path_1: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    pub auth_path_2: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    pub merkle_rt_old: Option<[u8; 32]>,

    // new coin 1
    pub a_sk_1: Option<Scalar>,
    pub rho_1: Option<Scalar>,
    pub r_1: Option<Scalar>,
    pub s_1: Option<Scalar>,
    pub v_1: Option<Scalar>,

    // new coin 2
    pub a_sk_2: Option<Scalar>,
    pub rho_2: Option<Scalar>,
    pub r_2: Option<Scalar>,
    pub s_2: Option<Scalar>,
    pub v_2: Option<Scalar>,

    pub constants: Vec<Scalar>,
}

fn make_test_context() -> (
    MerkleTree, // mt,
    // new coin 1
    Scalar, // a_sk_1,
    Scalar, // r_1,
    Scalar, // s_1,
    Scalar, // rho_1,
    Scalar, // v_1,
    Scalar, // a_pk_1,
    Scalar, // sn_1,
    Scalar, // k_1,
    Scalar, // cm_1,
    // new coin 2
    Scalar, // a_sk_2,
    Scalar, // r_2,
    Scalar, // s_2,
    Scalar, // rho_2,
    Scalar, // v_2,
    Scalar, // a_pk_2,
    Scalar, // sn_2,
    Scalar, // k_2,
    Scalar, // cm_2,
) {
    let (a_sk_1, r_1, s_1, rho_1, v_1, pk_1, sn_1, k_1, cm_1) = {
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        let random_u64 = OsRng.next_u64();

        let hasher = Hasher::new();

        let a_sk_1 = Scalar::from(random_u64);
        let r_1 = Scalar::from(6);
        let s_1 = Scalar::from(5);
        let rho_1 = Scalar::from(7);
        let v_1 = Scalar::from(100); // 100 sak
        let a_pk_1 = hasher.prf(Scalar::from(0), a_sk_1);
        let sn_1 = hasher.prf(a_sk_1, rho_1);
        let k_1 = hasher.comm(r_1, hasher.prf(a_pk_1, rho_1));
        let cm_1 = hasher.comm(s_1, hasher.prf(v_1, k_1));

        (a_sk_1, r_1, s_1, rho_1, v_1, a_pk_1, sn_1, k_1, cm_1)
    };

    let (a_sk_2, r_2, s_2, rho_2, v_2, pk_2, sn_2, k_2, cm_2) = {
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        let random_u64 = OsRng.next_u64();

        let hasher = Hasher::new();

        let a_sk_2 = Scalar::from(random_u64);
        let r_2 = Scalar::from(6);
        let s_2 = Scalar::from(5);
        let rho_2 = Scalar::from(7);
        let v_2 = Scalar::from(200); // 200 sak
        let a_pk_2 = hasher.prf(Scalar::from(0), a_sk_2);
        let sn_2 = hasher.prf(a_sk_2, rho_2);
        let k_2 = hasher.comm(r_2, hasher.prf(a_pk_2, rho_2));
        let cm_2 = hasher.comm(s_2, hasher.prf(v_2, k_2));

        (a_sk_2, r_2, s_2, rho_2, v_2, a_pk_2, sn_2, k_2, cm_2)
    };

    let constants = mimc::get_mimc_constants();

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];

    let mt = MerkleTree::new(3, &constants);

    (
        mt, // [gen_proof] : merkle_tree
        //
        a_sk_1, // [gen_proof] : secret key
        r_1,    // [gen_proof] : random sample value `r`
        s_1,    // [gen_proof] : random sample value `s`
        rho_1,  // [gen_proof] : rho value
        v_1,    // [gen_proof] : value of coin `v`
        pk_1,   // [ver_proof] : public key
        sn_1,   // [ver_proof] : serial number
        k_1,    // [ver_proof] : middle value (commitment) `k`
        cm_1,   // [ver_proof] : commitment `cm`
        //
        a_sk_2, // [gen_proof] : secret key
        r_2,    // [gen_proof] : random sample value `r`
        s_2,    // [gen_proof] : random sample value `s`
        rho_2,  // [gen_proof] : rho value
        v_2,    // [gen_proof] : value of coin `v`
        pk_2,   // [ver_proof] : public key
        sn_2,   // [ver_proof] : serial number
        k_2,    // [ver_proof] : middle value (commitment) `k`
        cm_2,   // [ver_proof] : commitment `cm`
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
            let c = TestCoinCircuit {
                hasher,

                // old coins
                cm_old_1: None,
                cm_old_2: None,
                auth_path_1: [None; TEST_TREE_DEPTH],
                auth_path_2: [None; TEST_TREE_DEPTH],
                merkle_rt_old: None,

                // new coin 1
                a_sk_1: None,
                rho_1: None,
                r_1: None,
                s_1: None,
                v_1: None,

                // new coin 2
                a_sk_2: None,
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
    cm_old_1: [u8; 32],
    cm_old_2: [u8; 32],
    auth_path_1: Vec<Path>,
    auth_path_2: Vec<Path>,
    merkle_rt_old: [u8; 32],

    // new coin 1
    a_sk_1: Scalar,
    rho_1: Scalar,
    r_1: Scalar,
    s_1: Scalar,
    v_1: Scalar,

    // new coin 1
    a_sk_2: Scalar,
    rho_2: Scalar,
    r_2: Scalar,
    s_2: Scalar,
    v_2: Scalar,
) -> Result<Proof<Bls12>, ProofError> {
    let constants = mimc::get_mimc_constants();
    let de_params = get_params_test(&constants);

    let a_sk_1 = Some(a_sk_1);
    let rho_1 = Some(rho_1);
    let r_1 = Some(r_1);
    let s_1 = Some(s_1);
    let v_1 = Some(v_1);
    //
    let a_sk_2 = Some(a_sk_2);
    let rho_2 = Some(rho_2);
    let r_2 = Some(r_2);
    let s_2 = Some(s_2);
    let v_2 = Some(v_2);

    let hasher = Hasher::new();

    let c = TestCoinCircuit {
        hasher,
        //
        a_sk_1,
        rho_1,
        r_1,
        s_1,
        v_1,
        //
        a_sk_2,
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

impl Circuit<Scalar> for TestCoinCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut rt = self.leaf.or(Some(Scalar::default()));

        let a_sk_1 = self.a_sk_1.or(Some(Scalar::default()));
        let rho_1 = self.rho_1.or(Some(Scalar::default()));
        let r_1 = self.r_1.or(Some(Scalar::default()));
        let s_1 = self.s_1.or(Some(Scalar::default()));
        let v_1 = self.v_1.or(Some(Scalar::default()));

        let a_sk_2 = self.a_sk_2.or(Some(Scalar::default()));
        let rho_2 = self.rho_2.or(Some(Scalar::default()));
        let r_2 = self.r_2.or(Some(Scalar::default()));
        let s_2 = self.s_2.or(Some(Scalar::default()));
        let v_2 = self.v_2.or(Some(Scalar::default()));

        // rt
        {
            for (idx, layer) in self.auth_path.iter().enumerate() {
                println!("idx: {}, layer: {:?}", idx, layer);

                let cs = &mut cs.namespace(|| format!("layer {}", idx));

                let cur_is_right = AllocatedBit::alloc(
                    cs.namespace(|| "cur is right"),
                    layer.as_ref().map(|&(_, d)| d),
                )
                .unwrap();

                let xl_value;
                let xr_value;

                let is_right = cur_is_right.get_value().and_then(|v| {
                    if v {
                        Some(true)
                    } else {
                        Some(false)
                    }
                });

                let temp = match *layer {
                    Some(a) => a,
                    None => (Scalar::default(), false),
                };

                if match is_right {
                    Some(a) => a,
                    None => false,
                } {
                    xl_value = Some(temp.0);
                    xr_value = rt;
                } else {
                    xl_value = rt;
                    xr_value = Some(temp.0);
                }

                rt = mimc::mimc_cs(cs, xl_value, xr_value, &self.constants);
            }
        };

        let (a_pk_1, sn_1, k_1, cm_1) = {
            // pk == PRF(a_sk, 0)
            let a_pk_1: Option<Scalar> =
                self.hasher.prf_cs(cs, Some(Scalar::from(0)), a_sk_1);

            // sn == PRF(a_sk, rho)
            let sn_1: Option<Scalar> = self.hasher.prf_cs(cs, a_sk_1, rho_1);

            // k == COMM(r, PRF(a_pk, rho))
            let k_1_tmp: Option<Scalar> = self.hasher.prf_cs(cs, a_pk_1, rho_1);
            let k_1: Option<Scalar> = self.hasher.comm_cs(cs, r_1, k_1_tmp);

            // cm == COMM(s, PRF(v, k))
            let cm_1_tmp: Option<Scalar> = self.hasher.prf_cs(cs, v_1, k_1);
            let cm_1: Option<Scalar> = self.hasher.comm_cs(cs, s_1, cm_1_tmp);

            (a_pk_1, sn_1, k_1, cm_1)
        };

        let (a_pk_2, sn_2, k_2, cm_2) = {
            // pk == PRF(a_sk, 0)
            let a_pk_2: Option<Scalar> =
                self.hasher.prf_cs(cs, Some(Scalar::from(0)), a_sk_2);

            // sn == PRF(a_sk, rho)
            let sn_2: Option<Scalar> = self.hasher.prf_cs(cs, a_sk_2, rho_2);

            // k == COMM(r, PRF(a_pk, rho))
            let k_2_tmp: Option<Scalar> = self.hasher.prf_cs(cs, a_pk_2, rho_2);
            let k_2: Option<Scalar> = self.hasher.comm_cs(cs, r_2, k_2_tmp);

            // cm == COMM(s, PRF(v, k))
            let cm_2_tmp: Option<Scalar> = self.hasher.prf_cs(cs, v_2, k_2);
            let cm_2: Option<Scalar> = self.hasher.comm_cs(cs, s_2, cm_2_tmp);

            (a_pk_2, sn_2, k_2, cm_2)
        };

        {
            cs.alloc_input(
                || "rt",
                || rt.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "a_pk_1",
                || a_pk_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_1",
                || sn_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                //
                || "k_1",
                || k_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_1",
                || cm_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "a_pk_2",
                || a_pk_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_2",
                || sn_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                //
                || "k_2",
                || k_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_2",
                || cm_2.ok_or(SynthesisError::AssignmentMissing),
            )?;
        }

        println!();
        println!("[+] Final values from test circuit :");
        println!("<1> rt: {:?}", rt);
        //
        println!("<2> a_pk_1: {:?}", a_pk_1);
        println!("<3> sn_1: {:?}", sn_1);
        println!("<4> k_1:  {:?}", k_1);
        println!("<5> cm_1: {:?}", cm_1);
        //
        println!("<6> a_pk_2: {:?}", a_pk_2);
        println!("<7> sn_2: {:?}", sn_2);
        println!("<8> k_2:  {:?}", k_2);
        println!("<9> cm_2: {:?}", cm_2);

        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default() {
    // sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    sak_test_utils::init_test_log();

    println!("[!] test coin ownership start!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    println!("\n[+] Test Context creating");

    let (
        mt, // merkle tree
        //
        a_sk_1, // secret key
        r_1,    // random sample value `r`
        s_1,    // random sample value `s`
        rho_1,  // rho value
        v_1,    // value of coin `v`
        a_pk_1, // public key
        sn_1,   // serial number
        k_1,    // middle value (commitment) `k`
        cm_1,   // commitment `cm`
        //
        a_sk_2, // secret key
        r_2,    // random sample value `r`
        s_2,    // random sample value `s`
        rho_2,  // rho value
        v_2,    // value of coin `v`
        a_pk_2, // public key
        sn_2,   // serial number
        k_2,    // middle value (commitment) `k`
        cm_2,   // commitment `cm`
    ) = make_test_context();

    let rt = mt.get_root().hash; // root hash value

    let tgt_leaf_idx = 0;

    println!("\n[+] Test Proof calculating");

    let proof = make_proof(
        tgt_leaf_idx,
        mt,
        //
        a_sk_1,
        rho_1,
        r_1,
        s_1,
        v_1,
        //
        a_sk_2,
        rho_2,
        r_2,
        s_2,
        v_2,
    )
    .unwrap();

    println!("\n[+] Test Verificationn");
    let public_inputs: Vec<Scalar> = vec![
        rt, //
        a_pk_1, sn_1, k_1, cm_1, //
        a_pk_2, sn_2, k_2, cm_2,
    ];
    let result = verify_proof(proof, &public_inputs);

    assert!(result);
}
