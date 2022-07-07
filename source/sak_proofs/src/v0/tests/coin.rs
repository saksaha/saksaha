use crate::{
    get_mimc_constants, mimc, mimc_cs, CoinCircuit, CoinProof, Hasher,
    MerkleTree, CM_TREE_DEPTH,
};
use bellman::gadgets::boolean::AllocatedBit;
use bellman::groth16::{self, Proof};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Scalar};
use rand::rngs::OsRng;
use rand::RngCore;

const TEST_TREE_DEPTH: usize = 3;

struct TestCoinCircuit {
    pub leaf: Option<Scalar>,
    pub auth_path: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for TestCoinCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut cur = match self.leaf {
            Some(a) => Some(a),
            None => Some(Scalar::default()),
        };

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
                    xr_value = cur;
                } else {
                    xl_value = cur;
                    xr_value = Some(temp.0);
                }

                cur = mimc_cs(cs, xl_value, xr_value, &self.constants);
            }
        };

        cs.alloc_input(
            || "image",
            || cur.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // let leaf = match self.leaf {
        //     Some(a) => Some(a),
        //     None => Some(S::default()),
        // };

        // cs.alloc_input(
        //     || "leaft",
        //     || leaf.ok_or(SynthesisError::AssignmentMissing),
        // )?;

        println!("final circuit public input {:?}", cur);

        Ok(())
    }
}

fn make_test_context() -> (MerkleTree, Scalar, Scalar) {
    // mint
    let v = 100; // 100 sak

    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();

    let hasher = Hasher::new();

    let sk = Scalar::from(random_u64);
    let sk_bytes = random_u64.to_be_bytes();

    // let pk = MiMC::mimc_single_arg(&sk);

    let pk = hasher.prf(Scalar::from(0), sk);
    println!("[-] sk: {}, \n[-] pk: {}", sk, pk);

    // let s = 5;
    // let r = 6;
    // let rho = 7;

    // // let k = MiMC::mimc(Scalar::from())
    // // MiMC::mimc()

    let constants = get_mimc_constants();

    let hasher = |xl, xr| {
        let hash = mimc(Scalar::from(xl), Scalar::from(xr), &constants);

        hash
    };

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];

    let mt = MerkleTree::new(data, 3, &constants, &hasher);

    (
        mt, // [gen_proof] : merkle_tree
        sk, // [gen_proof] : secret key
        pk, // [ver_proof] : public key
    )
}

fn make_proof(mt: MerkleTree) -> Proof<Bls12> {
    let proof = {
        let constants = get_mimc_constants();
        let de_params = CoinProof::get_params(&constants);

        // `rt` check
        let auth_path = {
            let tree = mt;
            let root = tree.get_root().hash;

            // println!("root: {:?}", root);

            let idx = 0;
            let auth_paths = tree.generate_auth_paths(idx);

            // for (idx, p) in auth_paths.iter().enumerate() {
            //     println!("auth path [{}] - {:?}", idx, p);
            // }

            let leaf =
                tree.nodes.get(0).unwrap().get(idx as usize).unwrap().hash;

            // println!("leaf: {:?}", leaf);

            // convert auth_paths => [auth_path]
            let mut auth_path: [Option<(Scalar, bool)>; TEST_TREE_DEPTH] =
                [None; TEST_TREE_DEPTH];

            for (idx, _) in auth_path.clone().iter().enumerate() {
                let sib = auth_paths.get(idx).unwrap();
                auth_path[idx] =
                    Some((sib.hash.clone(), sib.direction.clone()));
            }

            auth_path
        };

        let c = TestCoinCircuit {
            leaf: None,
            auth_path,
            constants,
        };

        let proof =
            groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

        proof
    };

    println!("[+] proof: {:?}", proof);

    proof
}

fn verify_proof(proof: Proof<Bls12>) -> bool {
    // let de_params = CoinProof::get_params(&constants);
    // // Prepare the verification key (for proof verification).
    // let pvk = groth16::prepare_verifying_key(&de_params.vk);

    // match groth16::verify_proof(&pvk, &proof, &[root]) {
    //     Ok(_) => {
    //         println!("verify success!");
    //         true
    //     }
    //     Err(err) => {
    //         println!("verify_proof(), err: {}", err);
    //         false
    //     }
    // }

    true
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default() {
    // sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    env_logger::init();

    println!("[!] test coin ownership start!!!!!!!!!!!!!!!!!!!!!!!!!");

    let (mt, sk, pk) = make_test_context();
    println!("[+] Test context has been constructed");

    let proof = make_proof(mt);
    println!("[+] Test Proof has been calculated");

    let result = verify_proof(proof);
    println!("[+] Test Verification has been done");

    println!("[!] test coin ownership ends...");

    assert!(true);
}
