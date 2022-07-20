use crate::{CoinProof, CM_TREE_DEPTH};
use rand::rngs::OsRng;
use sak_crypto::groth16;
use sak_crypto::{mimc, Scalar};
use std::time::SystemTime;

#[test]
pub fn mimc_test() {
    // let proof0 = CoinProof::generate_proof(0);

    // assert!(CoinProof::verify_proof(&proof0));
}

#[test]
pub fn performance_test() {
    // // println!("start");
    // // // let test_leaves: Vec<u32> = (0..std::u32::MAX).map(|x| x).collect();
    // // let mut test_leaves: Vec<u32> = vec![];
    // // (0..32).for_each(|iter| {
    // //     test_leaves.push(iter.clone());
    // // });
    // // println!("before new tree");

    // // let mut rng = thread_rng();
    // // let constants = (0..MIMC_ROUNDS)
    // //     .map(|_| Scalar::random(&mut rng))
    // //     .collect::<Vec<_>>();
    // let constants = mimc::get_mimc_constants();
    // // println!("constants : {:?}", constants);

    // // let mut bytes_constants = constants.clone();
    // // let changed_constants: Vec<[u8; 32]> =
    // //     bytes_constants.iter().map(|a| a.to_bytes()).collect();
    // // println!("changed constants: {:?}", changed_constants);

    // let tree = CoinProof::get_merkle_tree(&constants);

    // println!("before generate proof");
    // let auth_paths = tree.generate_auth_paths(0);
    // let leaf = tree.nodes.get(0).unwrap().get(0).unwrap().hash;
    // let root = tree.get_root().hash;

    // println!("\nauth_paths: {:?}", auth_paths);
    // println!("\nroot: {:?}", root.to_bytes());

    // let now = SystemTime::now();
    // println!("timer start {:?}", now);
    // // let params = {
    // //     let c = MyCircuit {
    // //         leaf: None,
    // //         auth_path: [None; TREE_DEPTH],
    // //         constants: &constants,
    // //     };

    // //     groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
    // //         .unwrap()
    // // };

    // let param_time = SystemTime::now();
    // println!(
    //     "generate random parameter end {:?}",
    //     param_time.duration_since(now)
    // );

    // // let is_file_exist = std::path::Path::new("new_params").exists();
    // // println!("file exist status : {}", is_file_exist);
    // // // write param to file
    // // let mut file = File::create("new_params").unwrap();
    // // let mut v = vec![];

    // // params.write(&mut v).unwrap();

    // // println!("writed data len: {}", v.len());

    // // // write origin buf
    // // file.write_all(&v);

    // // let strings: Vec<String> = v.iter().map(|n| n.to_string()).collect();
    // // write!(file, "{}", strings.join(",")).unwrap();

    // // read param from file
    // // let contents = std::fs::read_to_string("params").unwrap();
    // // let v2 = std::fs::read("params_8").unwrap();

    // // let v2: Vec<u8> = contents
    // //     .split(",")
    // //     .map(|x| x.parse::<u8>().unwrap())
    // //     .collect();

    // // println!("parsed data len: {}", v.len());

    // // let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
    // let de_params = CoinProof::get_params(&constants);

    // // Prepare the verification key (for proof verification).
    // let pvk = groth16::prepare_verifying_key(&de_params.vk);
    // let pvk = CoinProof::make_verifying_key(&de_params.vk);

    // let verify_key_time = SystemTime::now();

    // println!(
    //     "prepare verify key {:?}",
    //     verify_key_time.duration_since(param_time)
    // );

    // // Create an instance of our circuit (with the preimage as a witness).
    // let mut auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH] =
    //     [None; CM_TREE_DEPTH];

    // for (idx, elem) in auth_path.clone().iter().enumerate() {
    //     let sib = auth_paths.get(idx).unwrap();
    //     auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
    // }

    // let auth_path_time = SystemTime::now();

    // println!(
    //     "auth_path generate time {:?}",
    //     auth_path_time.duration_since(verify_key_time)
    // );

    // let c = CoinCircuit {
    //     leaf: Some(leaf),
    //     auth_path,
    //     constants,
    // };

    // let circuit_time = SystemTime::now();

    // println!(
    //     "circuit time {:?}",
    //     circuit_time.duration_since(auth_path_time)
    // );

    // let multipacking_time = SystemTime::now();

    // println!(
    //     "multipacking time {:?}",
    //     multipacking_time.duration_since(circuit_time)
    // );

    // let proof =
    //     groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

    // let proof_time = SystemTime::now();

    // println!(
    //     "proof time {:?}",
    //     proof_time.duration_since(multipacking_time)
    // );

    // match groth16::verify_proof(&pvk, &proof, &[root]) {
    //     Ok(_) => (println!("veryfiy success!")),
    //     Err(err) => {
    //         println!("verify_proof(), err: {}", err);
    //     }
    // }
    // assert!(groth16::verify_proof(&pvk, &proof, &[root]).is_ok());

    // let verify_finish_time = SystemTime::now();

    // println!(
    //     "verify proof finished {:?}",
    //     verify_finish_time.duration_since(proof_time)
    // );
}
