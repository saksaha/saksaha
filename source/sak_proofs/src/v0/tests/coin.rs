use crate::{mimc, Hasher, MerkleTree, ProofError};
use bellman::gadgets::boolean::AllocatedBit;
use bellman::groth16::{self, Parameters, Proof};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Scalar};
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::File;
use std::io::Write;

const TEST_TREE_DEPTH: usize = 3;

struct TestCoinCircuit {
    pub hasher: Hasher,
    pub leaf: Option<Scalar>,
    pub auth_path: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    pub sk: Option<Scalar>,
    pub rho: Option<Scalar>,
    pub r: Option<Scalar>,
    pub s: Option<Scalar>,
    pub v: Option<Scalar>,
    pub constants: Vec<Scalar>,
}

fn make_test_context() -> (
    MerkleTree, // mt,
    Scalar,     // sk,
    Scalar,     // r,
    Scalar,     // s,
    Scalar,     // rho,
    Scalar,     // v,
    Scalar,     // pk,
    Scalar,     // sn,
    Scalar,     // k,
    Scalar,     // cm,
) {
    // mint
    let v = Scalar::from(100); // 100 sak

    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();

    let hasher = Hasher::new();

    let sk = Scalar::from(random_u64);
    // let sk_bytes = random_u64.to_be_bytes();

    // let pk = MiMC::mimc_single_arg(&sk);
    let pk = hasher.prf(Scalar::from(0), sk);

    let s = Scalar::from(5);
    let r = Scalar::from(6);
    let rho = Scalar::from(7);

    let sn = hasher.prf(sk, rho);

    let k = hasher.comm(r, hasher.prf(pk, rho));

    let cm = hasher.comm(s, hasher.prf(v, k));
    // // let k = MiMC::mimc(Scalar::from())
    // // MiMC::mimc()

    let constants = mimc::get_mimc_constants();

    let hasher = |xl, xr| {
        let hash = mimc::mimc(Scalar::from(xl), Scalar::from(xr), &constants);

        hash
    };

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];

    let mt = MerkleTree::new(data, 3, &constants, &hasher);

    (
        mt,  // [gen_proof] : merkle_tree
        sk,  // [gen_proof] : secret key
        r,   // [gen_proof] : random sample value `r`
        s,   // [gen_proof] : random sample value `s`
        rho, // [gen_proof] : rho value
        v,   // [gen_proof] : value of coin `v`
        pk,  // [ver_proof] : public key
        sn,  // [ver_proof] : serial number
        k,   // [ver_proof] : middle value (commitment) `k`
        cm,  // [ver_proof] : commitment `cm`
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
                leaf: None,
                auth_path: [None; TEST_TREE_DEPTH],
                sk: None,
                rho: None,
                r: None,
                s: None,
                v: None,
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
    mt: MerkleTree,
    sk: Scalar,
    rho: Scalar,
    r: Scalar,
    s: Scalar,
    v: Scalar,
) -> Result<Proof<Bls12>, ProofError> {
    let constants = mimc::get_mimc_constants();
    let de_params = get_params_test(&constants);

    // `rt` check
    let auth_path = {
        let tree = &mt;
        let root = tree.get_root().hash;

        println!("root: {:?}", root);

        let idx = 0;
        let auth_paths = tree.generate_auth_paths(idx);

        for (idx, p) in auth_paths.iter().enumerate() {
            println!("auth path [{}] - {:?}", idx, p);
        }

        let target_leaf =
            tree.nodes.get(0).unwrap().get(tgt_leaf_idx).unwrap().hash;

        println!("target_leaf: {:?}, idx: {}", target_leaf, idx);

        // convert auth_paths => [auth_path]
        let mut auth_path: [Option<(Scalar, bool)>; TEST_TREE_DEPTH] =
            [None; TEST_TREE_DEPTH];

        for (idx, _) in auth_path.clone().iter().enumerate() {
            let sib = auth_paths.get(idx).unwrap();
            auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
        }

        auth_path
    };

    let leaf = Some(mt.nodes.get(0).unwrap().get(tgt_leaf_idx).unwrap().hash);
    let sk = Some(sk);
    let rho = Some(rho);
    let r = Some(r);
    let s = Some(s);
    let v = Some(v);
    let hasher = Hasher::new();

    let c = TestCoinCircuit {
        hasher,
        leaf,
        auth_path,
        sk,
        rho,
        r,
        s,
        v,
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

        let sk = self.sk.or(Some(Scalar::default()));

        let rho = self.rho.or(Some(Scalar::default()));

        let r = self.r.or(Some(Scalar::default()));

        let s = self.s.or(Some(Scalar::default()));

        let v = self.v.or(Some(Scalar::default()));

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

        // pk == PRF(a_sk, 0)
        let pk: Option<Scalar> =
            self.hasher.prf_cs(cs, Some(Scalar::from(0)), sk);

        // sn == PRF(a_sk, rho)
        let sn: Option<Scalar> = self.hasher.prf_cs(cs, sk, rho);

        // k == COMM(r, PRF(a_pk, rho))
        let k_tmp: Option<Scalar> = self.hasher.prf_cs(cs, pk, rho);
        let k: Option<Scalar> = self.hasher.comm_cs(cs, r, k_tmp);

        // cm == COMM(s, PRF(v, k))
        let cm_tmp: Option<Scalar> = self.hasher.prf_cs(cs, v, k);
        let cm: Option<Scalar> = self.hasher.comm_cs(cs, s, cm_tmp);

        cs.alloc_input(
            || "rt",
            || rt.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.alloc_input(
            || "pk",
            || pk.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.alloc_input(
            || "sn",
            || sn.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.alloc_input(
            //
            || "k",
            || k.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.alloc_input(
            || "cm",
            || cm.ok_or(SynthesisError::AssignmentMissing),
        )?;

        println!();
        println!("[+] Final values from test circuit :");
        println!("<1> rt: {:?}", rt);
        println!("<2> pk: {:?}", pk);
        println!("<3> sn: {:?}", sn);
        println!("<4> k:  {:?}", k);
        println!("<4> cm: {:?}", cm);

        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default() {
    // sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    env_logger::init();

    println!("[!] test coin ownership start!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    println!("\n[+] Test Context creating");

    let (
        mt,  // merkle tree
        sk,  // secret key
        r,   // random sample value `r`
        s,   // random sample value `s`
        rho, // rho value
        v,   // value of coin `v`
        pk,  // public key
        sn,  // serial number
        k,   // middle value (commitment) `k`
        cm,  // commitment `cm`
    ) = make_test_context();

    let rt = mt.get_root().hash; // root hash value

    let tgt_leaf_idx = 0;

    println!("\n[+] Test Proof calculating");
    let proof = make_proof(tgt_leaf_idx, mt, sk, rho, r, s, v).unwrap();

    println!("\n[+] Test Verificationn");
    let public_inputs: Vec<Scalar> = vec![rt, pk, sn, k, cm];
    let result = verify_proof(proof, &public_inputs);

    assert!(result);
}
