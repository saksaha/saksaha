mod merkle;
mod mimc;
mod proofs;

#[cfg(test)]
mod tests;

pub use merkle::*;
pub use mimc::*;
pub use proofs::*;

// const TREE_DEPTH: usize = 5;
// // const TREE_DEPTH: usize = 32;
// pub const MIMC_ROUNDS: usize = 322;

// struct MyCircuit<'a, S: PrimeField> {
//     leaf: Option<S>,
//     auth_path: [Option<(S, bool)>; TREE_DEPTH],
//     constants: &'a [S],
// }

// impl<'a, S: PrimeField> Circuit<S> for MyCircuit<'a, S> {
//     fn synthesize<CS: ConstraintSystem<S>>(
//         self,
//         cs: &mut CS,
//     ) -> Result<(), SynthesisError> {
//         let mut cur = match self.leaf {
//             Some(a) => Some(a),
//             None => Some(S::default()),
//         };

//         {
//             for (idx, layer) in self.auth_path.iter().enumerate() {
//                 println!("idx: {}, layer: {:?}", idx, layer);

//                 let cs = &mut cs.namespace(|| format!("layer {}", idx));

//                 let cur_is_right = AllocatedBit::alloc(
//                     cs.namespace(|| "cur is right"),
//                     layer.as_ref().map(|&(_, d)| d),
//                 )
//                 .unwrap();

//                 // start mimc
//                 let mut xl_value;
//                 let mut xr_value;

//                 let is_right = cur_is_right.get_value().and_then(|v| {
//                     if v {
//                         Some(true)
//                     } else {
//                         Some(false)
//                     }
//                 });

//                 let temp = match *layer {
//                     Some(a) => a,
//                     None => (S::default(), false),
//                 };

//                 // cur_is_right
//                 if match is_right {
//                     Some(a) => a,
//                     None => false,
//                 } {
//                     xl_value = Some(temp.0);
//                     xr_value = cur;
//                 } else {
//                     xl_value = cur;
//                     xr_value = Some(temp.0);
//                 }

//                 println!("xl: {:?}, xr: {:?}", xl_value, xr_value);

//                 let mut xl = cs.alloc(
//                     || "preimage xl",
//                     || xl_value.ok_or(SynthesisError::AssignmentMissing),
//                 )?;

//                 // Allocate the second component of the preimage.
//                 // let mut xr_value = self.xr;
//                 let mut xr = cs.alloc(
//                     || "preimage xr",
//                     || xr_value.ok_or(SynthesisError::AssignmentMissing),
//                 )?;

//                 for i in 0..MIMC_ROUNDS {
//                     // xL, xR := xR + (xL + Ci)^3, xL
//                     // let cs = &mut cs.namespace(|| format!("round {}", i));

//                     // tmp = (xL + Ci)^2
//                     let tmp_value = xl_value.map(|mut e| {
//                         e.add_assign(&self.constants[i]);
//                         e.square()
//                     });
//                     let tmp = cs.alloc(
//                         || "tmp",
//                         || tmp_value.ok_or(SynthesisError::AssignmentMissing),
//                     )?;

//                     cs.enforce(
//                         || "tmp = (xL + Ci)^2",
//                         |lc| lc + xl + (self.constants[i], CS::one()),
//                         |lc| lc + xl + (self.constants[i], CS::one()),
//                         |lc| lc + tmp,
//                     );

//                     // new_xL = xR + (xL + Ci)^3
//                     // new_xL = xR + tmp * (xL + Ci)
//                     // new_xL - xR = tmp * (xL + Ci)
//                     let new_xl_value = xl_value.map(|mut e| {
//                         e.add_assign(&self.constants[i]);
//                         e.mul_assign(&tmp_value.unwrap());
//                         e.add_assign(&xr_value.unwrap());
//                         e
//                     });

//                     let new_xl = cs.alloc(
//                         || "new_xl",
//                         || {
//                             new_xl_value
//                                 .ok_or(SynthesisError::AssignmentMissing)
//                         },
//                     )?;

//                     cs.enforce(
//                         || "new_xL = xR + (xL + Ci)^3",
//                         |lc| lc + tmp,
//                         |lc| lc + xl + (self.constants[i], CS::one()),
//                         |lc| lc + new_xl - xr,
//                     );

//                     // xR = xL
//                     xr = xl;
//                     xr_value = xl_value;

//                     // xL = new_xL
//                     xl = new_xl;
//                     xl_value = new_xl_value;
//                 }

//                 cur = xl_value;
//                 // println!("circuit public input {:?}", cur.unwrap());
//                 // end of mimc

//                 // let cur_str = convert_to_str(cur.clone());
//                 // println!("\nlayer_idx: {}, cur: {}", idx, cur_str);
//             }
//         };

//         cs.alloc_input(
//             || "image",
//             || cur.ok_or(SynthesisError::AssignmentMissing),
//         )?;

//         let leaf = match self.leaf {
//             Some(a) => Some(a),
//             None => Some(S::default()),
//         };

//         cs.alloc_input(
//             || "image",
//             || leaf.ok_or(SynthesisError::AssignmentMissing),
//         )?;

//         println!("final circuit public input {:?}", cur);

//         Ok(())
//     }
// }

// pub fn mimc<S: PrimeField>(mut xl: S, mut xr: S, constants: &[S]) -> S {
//     assert_eq!(constants.len(), MIMC_ROUNDS);

//     for c in constants {
//         let mut tmp1 = xl;
//         tmp1.add_assign(c);
//         let mut tmp2 = tmp1.square();
//         tmp2.mul_assign(&tmp1);
//         tmp2.add_assign(&xr);
//         xr = xl;
//         xl = tmp2;
//     }

//     xl
// }

// pub fn get_params(constants: &[Scalar]) -> Parameters<Bls12> {
//     let is_file_exist = std::path::Path::new("mimc_params").exists();
//     // println!("file exist status : {}", is_file_exist);
//     let mut v = vec![];
//     if is_file_exist {
//         // read
//         v = std::fs::read("mimc_params").unwrap();
//     } else {
//         // generate and write
//         let params = {
//             let c = MyCircuit {
//                 leaf: None,
//                 auth_path: [None; TREE_DEPTH],
//                 constants: &constants,
//             };

//             groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
//                 .unwrap()
//         };
//         // write param to file
//         let mut file = File::create("mimc_params").unwrap();

//         params.write(&mut v).unwrap();
//         // write origin buf
//         file.write_all(&v);
//     }

//     println!("params len: {}", v.len());

//     let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
//     de_params
// }

// pub fn get_merkle_tree(constants: &[Scalar]) -> Tree {
//     let mut leaves: Vec<u32> = vec![];

//     (0..32).for_each(|iter| {
//         leaves.push(iter.clone());
//     });

//     let tree = Tree::new(leaves, TREE_DEPTH, &constants);
//     tree
// }

// pub fn generate_proof(idx: usize) -> Proof<Bls12> {
//     let constants = get_round_constants();

//     let tree = get_merkle_tree(&constants);

//     // make auth_paths and leaf of {idx}
//     let auth_paths = tree.generate_auth_paths(idx.try_into().unwrap());

//     // println!("auth path ({}): {:?}", auth_paths.len(), auth_paths);
//     for (idx, p) in auth_paths.iter().enumerate() {
//         println!("auth path [{}] - {:?}", idx, p);
//     }

//     let leaf = tree.nodes.get(0).unwrap().get(idx).unwrap().hash;

//     println!("leaf: {:?}", leaf);

//     let de_params = get_params(&constants);

//     // convert auth_paths => [auth_path]
//     let mut auth_path: [Option<(Scalar, bool)>; TREE_DEPTH] =
//         [None; TREE_DEPTH];

//     for (idx, _) in auth_path.clone().iter().enumerate() {
//         let sib = auth_paths.get(idx).unwrap();
//         auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
//     }

//     let c = MyCircuit {
//         leaf: Some(leaf),
//         auth_path,
//         constants: &constants,
//     };

//     let proof =
//         groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

//     proof
// }

// pub fn verify_proof(proof: &Proof<Bls12>) -> bool {
//     let constants = get_round_constants();

//     let de_params = get_params(&constants);

//     let tree = get_merkle_tree(&constants);

//     let root = tree.root().hash;

//     let leaf = tree.nodes.get(0).unwrap().get(0).unwrap().hash;

//     // Prepare the verification key (for proof verification).
//     let pvk = groth16::prepare_verifying_key(&de_params.vk);

//     match groth16::verify_proof(&pvk, &proof, &[root, leaf]) {
//         Ok(_) => {
//             println!("verify success!");
//             true
//         }
//         Err(err) => {
//             println!("verify_proof(), err: {}", err);
//             false
//         }
//     }
// }

// #[test]
// pub fn mimc_test() {
//     let proof0 = generate_proof(0);

//     assert!(verify_proof(&proof0));

//     // let proof12 = generate_proof(12);
//     // assert!(verify_proof(proof12));
// }

// #[test]
// pub fn performance_test() {
//     // println!("start");
//     // // let test_leaves: Vec<u32> = (0..std::u32::MAX).map(|x| x).collect();
//     // let mut test_leaves: Vec<u32> = vec![];
//     // (0..32).for_each(|iter| {
//     //     test_leaves.push(iter.clone());
//     // });
//     // println!("before new tree");

//     // let mut rng = thread_rng();
//     // let constants = (0..MIMC_ROUNDS)
//     //     .map(|_| Scalar::random(&mut rng))
//     //     .collect::<Vec<_>>();
//     let constants = get_round_constants();
//     println!("constants : {:?}", constants);

//     // let mut bytes_constants = constants.clone();
//     // let changed_constants: Vec<[u8; 32]> =
//     //     bytes_constants.iter().map(|a| a.to_bytes()).collect();
//     // println!("changed constants: {:?}", changed_constants);

//     let tree = get_merkle_tree(&constants);

//     println!("before generate proof");
//     let auth_paths = tree.generate_auth_paths(0);
//     let leaf = tree.nodes.get(0).unwrap().get(0).unwrap().hash;
//     let root = tree.root().hash;

//     println!("\nauth_paths: {:?}", auth_paths);
//     println!("\nroot: {:?}", root.to_bytes());

//     let now = SystemTime::now();
//     println!("timer start {:?}", now);
//     // let params = {
//     //     let c = MyCircuit {
//     //         leaf: None,
//     //         auth_path: [None; TREE_DEPTH],
//     //         constants: &constants,
//     //     };

//     //     groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
//     //         .unwrap()
//     // };

//     let param_time = SystemTime::now();
//     println!(
//         "generate random parameter end {:?}",
//         param_time.duration_since(now)
//     );

//     // let is_file_exist = std::path::Path::new("new_params").exists();
//     // println!("file exist status : {}", is_file_exist);
//     // // write param to file
//     // let mut file = File::create("new_params").unwrap();
//     // let mut v = vec![];

//     // params.write(&mut v).unwrap();

//     // println!("writed data len: {}", v.len());

//     // // write origin buf
//     // file.write_all(&v);

//     // let strings: Vec<String> = v.iter().map(|n| n.to_string()).collect();
//     // write!(file, "{}", strings.join(",")).unwrap();

//     // read param from file
//     // let contents = std::fs::read_to_string("params").unwrap();
//     // let v2 = std::fs::read("params_8").unwrap();

//     // let v2: Vec<u8> = contents
//     //     .split(",")
//     //     .map(|x| x.parse::<u8>().unwrap())
//     //     .collect();

//     // println!("parsed data len: {}", v.len());

//     // let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
//     let de_params = get_params(&constants);

//     // Prepare the verification key (for proof verification).
//     let pvk = groth16::prepare_verifying_key(&de_params.vk);

//     let verify_key_time = SystemTime::now();
//     println!(
//         "prepare verify key {:?}",
//         verify_key_time.duration_since(param_time)
//     );

//     // Create an instance of our circuit (with the preimage as a witness).
//     let mut auth_path: [Option<(Scalar, bool)>; TREE_DEPTH] =
//         [None; TREE_DEPTH];
//     for (idx, elem) in auth_path.clone().iter().enumerate() {
//         let sib = auth_paths.get(idx).unwrap();
//         auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
//     }
//     let auth_path_time = SystemTime::now();
//     println!(
//         "auth_path generate time {:?}",
//         auth_path_time.duration_since(verify_key_time)
//     );

//     let c = MyCircuit {
//         leaf: Some(leaf),
//         // leaf: Some(Scalar::from_bytes(&leaf).unwrap()),
//         auth_path: auth_path,
//         constants: &constants,
//     };
//     let circuit_time = SystemTime::now();
//     println!(
//         "circuit time {:?}",
//         circuit_time.duration_since(auth_path_time)
//     );

//     // let root_bits = multipack::bytes_to_bits_le(&root);
//     // let inputs = multipack::compute_multipacking(&root_bits);
//     let multipacking_time = SystemTime::now();
//     println!(
//         "multipacking time {:?}",
//         multipacking_time.duration_since(circuit_time)
//     );

//     let proof =
//         groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();
//     let proof_time = SystemTime::now();
//     println!(
//         "proof time {:?}",
//         proof_time.duration_since(multipacking_time)
//     );

//     match groth16::verify_proof(&pvk, &proof, &[root]) {
//         Ok(_) => (println!("veryfiy success!")),
//         Err(err) => {
//             println!("verify_proof(), err: {}", err);
//         }
//     }
//     assert!(groth16::verify_proof(&pvk, &proof, &[root]).is_ok());

//     let verify_finish_time = SystemTime::now();
//     println!(
//         "verify proof finished {:?}",
//         verify_finish_time.duration_since(proof_time)
//     );
// }
