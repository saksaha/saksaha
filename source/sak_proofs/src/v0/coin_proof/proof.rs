use crate::CoinCircuit;
use crate::MerkleTree;
use crate::CM_TREE_DEPTH;
use pairing::MultiMillerLoop;
use rand::rngs::OsRng;
use sak_crypto::{
    groth16, mimc, AllocatedBit, Bls12, Circuit, ConstraintSystem,
    Groth16VerifyingKey, Parameters, PreparedVerifyingKey, Proof, Scalar,
    SynthesisError,
};
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
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

        println!("Wll");
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
