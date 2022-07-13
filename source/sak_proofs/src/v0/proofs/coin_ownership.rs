use crate::MerkleTree;
use bellman::gadgets::boolean::AllocatedBit;
use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof, VerifyingKey};
use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use pairing::MultiMillerLoop;
use rand::rngs::OsRng;
use sak_crypto::{mimc, Bls12, Scalar};
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

//
pub const CM_TREE_DEPTH: usize = 5;

pub const CM_TREE_CAPACITY: usize = 2_usize.pow(CM_TREE_DEPTH as u32);

// const MIMC_ROUNDS: usize = 322;

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }

    // TODO use sak::fs
    pub fn get_params(constants: &[Scalar]) -> Parameters<Bls12> {
        let is_file_exist = std::path::Path::new("mimc_params").exists();
        let mut v = vec![];
        if is_file_exist {
            // read
            v = std::fs::read("mimc_params").unwrap();
        } else {
            // generate and write
            let params = {
                let c = CoinCircuit {
                    leaf: None,
                    auth_path: [None; CM_TREE_DEPTH],
                    constants: constants.to_vec(),
                };

                groth16::generate_random_parameters::<Bls12, _, _>(
                    c, &mut OsRng,
                )
                .unwrap()
            };
            // write param to file
            let mut file = File::create("mimc_params").unwrap();

            params.write(&mut v).unwrap();
            // write origin buf
            file.write_all(&v);
        }

        println!("params len: {}", v.len());

        let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
        de_params
    }

    pub fn get_merkle_tree(constants: &[Scalar]) -> MerkleTree {
        let mut leaves: Vec<u32> = vec![];

        (0..32).for_each(|iter| {
            leaves.push(iter.clone());
        });

        let hasher = |xl, xr| {
            let hash =
                mimc::mimc(Scalar::from(xl), Scalar::from(xr), constants);

            hash
        };

        let tree = MerkleTree::new(leaves, CM_TREE_DEPTH, &constants, &hasher);
        tree
    }

    pub fn generate_proof(idx: usize) -> Proof<Bls12> {
        let constants = mimc::get_mimc_constants();

        let tree = CoinProof::get_merkle_tree(&constants);

        // make auth_paths and leaf of {idx}
        let auth_paths = tree.generate_auth_paths(idx.try_into().unwrap());

        // println!("auth path ({}): {:?}", auth_paths.len(), auth_paths);
        for (idx, p) in auth_paths.iter().enumerate() {
            println!("auth path [{}] - {:?}", idx, p);
        }

        let leaf = tree.nodes.get(0).unwrap().get(idx).unwrap().hash;

        println!("leaf: {:?}", leaf);

        let de_params = CoinProof::get_params(&constants);

        // convert auth_paths => [auth_path]
        let mut auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH] =
            [None; CM_TREE_DEPTH];

        for (idx, _) in auth_path.clone().iter().enumerate() {
            let sib = auth_paths.get(idx).unwrap();
            auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
        }

        let c = CoinCircuit {
            leaf: Some(leaf),
            auth_path,
            constants,
        };

        let proof =
            groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

        proof
    }

    pub fn verify_proof(proof: &Proof<Bls12>) -> bool {
        let constants = mimc::get_mimc_constants();

        let de_params = CoinProof::get_params(&constants);

        let tree = CoinProof::get_merkle_tree(&constants);

        let root = tree.get_root().hash;

        let leaf = tree.nodes.get(0).unwrap().get(0).unwrap().hash;

        // Prepare the verification key (for proof verification).
        let pvk = groth16::prepare_verifying_key(&de_params.vk);

        match groth16::verify_proof(&pvk, &proof, &[root]) {
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
}

pub struct CoinCircuit {
    pub leaf: Option<Scalar>,
    pub auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH],
    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for CoinCircuit {
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

                cur = mimc::mimc_cs(cs, xl_value, xr_value, &self.constants);
            }
        };

        cs.alloc_input(
            || "image",
            || cur.ok_or(SynthesisError::AssignmentMissing),
        )?;

        println!("final circuit public input {:?}", cur);

        Ok(())
    }
}
