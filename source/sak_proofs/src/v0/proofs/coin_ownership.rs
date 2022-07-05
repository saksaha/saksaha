use crate::{MerkleTree, MiMC};
use bellman::gadgets::boolean::AllocatedBit;
use bellman::groth16::{Parameters, PreparedVerifyingKey, Proof, VerifyingKey};
use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::Bls12;
use bls12_381::Scalar;
use ff::PrimeField;
use pairing::MultiMillerLoop;
use rand::rngs::OsRng;
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

//
pub const TREE_DEPTH: usize = 5;

//
pub const MIMC_ROUNDS: usize = 322;

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
                    auth_path: [None; TREE_DEPTH],
                    constants: &constants,
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

        let tree = MerkleTree::new(leaves, TREE_DEPTH, &constants);
        tree
    }

    pub fn generate_proof(idx: usize) -> Proof<Bls12> {
        let constants = MiMC::get_mimc_constants();

        let tree = Self::get_merkle_tree(&constants);

        // make auth_paths and leaf of {idx}
        let auth_paths = tree.generate_auth_paths(idx.try_into().unwrap());

        // println!("auth path ({}): {:?}", auth_paths.len(), auth_paths);
        for (idx, p) in auth_paths.iter().enumerate() {
            println!("auth path [{}] - {:?}", idx, p);
        }

        let leaf = tree.nodes.get(0).unwrap().get(idx).unwrap().hash;

        println!("leaf: {:?}", leaf);

        let de_params = Self::get_params(&constants);

        // convert auth_paths => [auth_path]
        let mut auth_path: [Option<(Scalar, bool)>; TREE_DEPTH] =
            [None; TREE_DEPTH];

        for (idx, _) in auth_path.clone().iter().enumerate() {
            let sib = auth_paths.get(idx).unwrap();
            auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
        }

        let c = CoinCircuit {
            leaf: Some(leaf),
            auth_path,
            constants: &constants,
        };

        let proof =
            groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

        proof
    }

    pub fn verify_proof(proof: &Proof<Bls12>) -> bool {
        let constants = MiMC::get_mimc_constants();

        let de_params = Self::get_params(&constants);

        let tree = Self::get_merkle_tree(&constants);

        let root = tree.root().hash;

        let leaf = tree.nodes.get(0).unwrap().get(0).unwrap().hash;

        // Prepare the verification key (for proof verification).
        let pvk = groth16::prepare_verifying_key(&de_params.vk);

        match groth16::verify_proof(&pvk, &proof, &[root, leaf]) {
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

pub struct CoinCircuit<'a, S: PrimeField> {
    pub leaf: Option<S>,
    pub auth_path: [Option<(S, bool)>; TREE_DEPTH],
    pub constants: &'a [S],
}

impl<'a, S: PrimeField> Circuit<S> for CoinCircuit<'a, S> {
    fn synthesize<CS: ConstraintSystem<S>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut cur = match self.leaf {
            Some(a) => Some(a),
            None => Some(S::default()),
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

                // start mimc
                let mut xl_value;
                let mut xr_value;

                let is_right = cur_is_right.get_value().and_then(|v| {
                    if v {
                        Some(true)
                    } else {
                        Some(false)
                    }
                });

                let temp = match *layer {
                    Some(a) => a,
                    None => (S::default(), false),
                };

                // cur_is_right
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

                println!("xl: {:?}, xr: {:?}", xl_value, xr_value);

                let mut xl = cs.alloc(
                    || "preimage xl",
                    || xl_value.ok_or(SynthesisError::AssignmentMissing),
                )?;

                // Allocate the second component of the preimage.
                // let mut xr_value = self.xr;
                let mut xr = cs.alloc(
                    || "preimage xr",
                    || xr_value.ok_or(SynthesisError::AssignmentMissing),
                )?;

                for i in 0..MIMC_ROUNDS {
                    // xL, xR := xR + (xL + Ci)^3, xL
                    // let cs = &mut cs.namespace(|| format!("round {}", i));

                    // tmp = (xL + Ci)^2
                    let tmp_value = xl_value.map(|mut e| {
                        e.add_assign(&self.constants[i]);
                        e.square()
                    });
                    let tmp = cs.alloc(
                        || "tmp",
                        || tmp_value.ok_or(SynthesisError::AssignmentMissing),
                    )?;

                    cs.enforce(
                        || "tmp = (xL + Ci)^2",
                        |lc| lc + xl + (self.constants[i], CS::one()),
                        |lc| lc + xl + (self.constants[i], CS::one()),
                        |lc| lc + tmp,
                    );

                    // new_xL = xR + (xL + Ci)^3
                    // new_xL = xR + tmp * (xL + Ci)
                    // new_xL - xR = tmp * (xL + Ci)
                    let new_xl_value = xl_value.map(|mut e| {
                        e.add_assign(&self.constants[i]);
                        e.mul_assign(&tmp_value.unwrap());
                        e.add_assign(&xr_value.unwrap());
                        e
                    });

                    let new_xl = cs.alloc(
                        || "new_xl",
                        || {
                            new_xl_value
                                .ok_or(SynthesisError::AssignmentMissing)
                        },
                    )?;

                    cs.enforce(
                        || "new_xL = xR + (xL + Ci)^3",
                        |lc| lc + tmp,
                        |lc| lc + xl + (self.constants[i], CS::one()),
                        |lc| lc + new_xl - xr,
                    );

                    // xR = xL
                    xr = xl;
                    xr_value = xl_value;

                    // xL = new_xL
                    xl = new_xl;
                    xl_value = new_xl_value;
                }

                cur = xl_value;
                // println!("circuit public input {:?}", cur.unwrap());
                // end of mimc

                // let cur_str = convert_to_str(cur.clone());
                // println!("\nlayer_idx: {}, cur: {}", idx, cur_str);
            }
        };

        cs.alloc_input(
            || "image",
            || cur.ok_or(SynthesisError::AssignmentMissing),
        )?;

        let leaf = match self.leaf {
            Some(a) => Some(a),
            None => Some(S::default()),
        };

        cs.alloc_input(
            || "image",
            || leaf.ok_or(SynthesisError::AssignmentMissing),
        )?;
        println!("final circuit public input {:?}", cur);

        Ok(())
    }
}
