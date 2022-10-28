use sak_proof::{make_test_context_2_to_2, CoinProof};
use sak_proof_types::{NewCoin, OldCoin};
use wasm_bindgen::prelude::*;

use rayon::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(numbers: &[i32]) -> i32 {
    alert(&format!("array"));
    let result = numbers.par_iter().sum();
    alert(&format!("sum result: {}", result));

    result
}

// #[wasm_bindgen]
// pub fn greet_proof(name: &str) {
//     let test_context = make_test_context_2_to_2();

//     let coin_1_old = OldCoin {
//         addr_pk: Some(test_context.addr_pk_1_old),
//         addr_sk: Some(test_context.addr_sk_1_old),
//         rho: Some(test_context.rho_1_old),
//         r: Some(test_context.r_1_old),
//         s: Some(test_context.s_1_old),
//         v: Some(test_context.v_1_old),
//         cm: Some(test_context.cm_1_old),
//         auth_path: test_context.auth_path_1.map(|e| Some(e)),
//     };

//     let coin_2_old = OldCoin {
//         addr_pk: Some(test_context.addr_pk_2_old),
//         addr_sk: Some(test_context.addr_sk_2_old),
//         rho: Some(test_context.rho_2_old),
//         r: Some(test_context.r_2_old),
//         s: Some(test_context.s_2_old),
//         v: Some(test_context.v_2_old),
//         cm: Some(test_context.cm_2_old),
//         auth_path: test_context.auth_path_2.map(|e| Some(e)),
//     };

//     let coin_1_new = NewCoin {
//         addr_pk: Some(test_context.addr_pk_1),
//         rho: Some(test_context.rho_1),
//         r: Some(test_context.r_1),
//         s: Some(test_context.s_1),
//         v: Some(test_context.v_1),
//     };

//     let coin_2_new = NewCoin {
//         addr_pk: Some(test_context.addr_pk_2),
//         rho: Some(test_context.rho_2),
//         r: Some(test_context.r_2),
//         s: Some(test_context.s_2),
//         v: Some(test_context.v_2),
//     };

//     // Here success
//     alert(&format!("Hello 333"));

//     let start = std::time::SystemTime::now();
//     alert(&format!("start: {:?}", start));
//     let proof =
//         match CoinProof::generate_proof_2_to_2(coin_1_old, coin_2_old, coin_1_new, coin_2_new) {
//             Ok(v) => v,
//             Err(err) => {
//                 alert(&format!("failed to generate proof: {:?}", err.to_string()));
//                 panic!();
//             }
//         };

//     // let end = std::time::SystemTime::now();
//     // alert(&format!("end: {:?}", end));

//     // let pi_serialized = CoinProof::serialize_pi(&proof).unwrap();

//     // alert(&format!("Hello, {:?}!, my pi: {:?}", name, pi_serialized));

//     // let mut key = [0u8; 16];
//     // OsRng.fill_bytes(&mut key);
//     // let random_u64 = OsRng.next_u64();

//     // alert(&format!("random_u64:{}", random_u64));

//     // // Create parameters for our circuit. In a production deployment these would
//     // // be generated securely using a multiparty computation.
//     // let params = {
//     //     let c = MyCircuit { preimage: None };
//     //     groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
//     // };

//     // alert(&format!("proof 1"));

//     // // Prepare the verification key (for proof verification).
//     // let pvk = groth16::prepare_verifying_key(&params.vk);

//     // // Pick a preimage and compute its hash.
//     // let preimage = [42; 80];
//     // let hash = Sha256::digest(&Sha256::digest(&preimage));

//     // alert(&format!("proof 2"));

//     // // Create an instance of our circuit (with the preimage as a witness).
//     // let c = MyCircuit {
//     //     preimage: Some(preimage),
//     // };

//     // alert(&format!("Hello 333"));
//     // // Create a Groth16 proof with our parameters.
//     // let proof = groth16::create_random_proof(c, &params, &mut OsRng).unwrap();

//     // alert(&format!("Hello 444"));

//     // // Pack the hash as inputs for proof verification.
//     // // let hash_bits = multipack::bytes_to_bits_le(&hash);
//     // // let inputs = multipack::compute_multipacking(&hash_bits);

//     // // Check the proof!
//     // // assert!(groth16::verify_proof(&pvk, &proof, &inputs).is_ok());
// }

// use bellman::{
//     gadgets::{
//         boolean::{AllocatedBit, Boolean},
//         multipack,
//         sha256::sha256,
//     },
//     groth16, Circuit, ConstraintSystem, SynthesisError,
// };
// use bls12_381::Bls12;
// use ff::PrimeField;
// use pairing::Engine;
// use sha2::{Digest, Sha256};

// /// Our own SHA-256d gadget. Input and output are in little-endian bit order.
// fn sha256d<Scalar: PrimeField, CS: ConstraintSystem<Scalar>>(
//     mut cs: CS,
//     data: &[Boolean],
// ) -> Result<Vec<Boolean>, SynthesisError> {
//     // Flip endianness of each input byte
//     let input: Vec<_> = data
//         .chunks(8)
//         .map(|c| c.iter().rev())
//         .flatten()
//         .cloned()
//         .collect();

//     let mid = sha256(cs.namespace(|| "SHA-256(input)"), &input)?;
//     let res = sha256(cs.namespace(|| "SHA-256(mid)"), &mid)?;

//     // Flip endianness of each output byte
//     Ok(res
//         .chunks(8)
//         .map(|c| c.iter().rev())
//         .flatten()
//         .cloned()
//         .collect())
// }

// struct MyCircuit {
//     /// The input to SHA-256d we are proving that we know. Set to `None` when we
//     /// are verifying a proof (and do not have the witness data).
//     preimage: Option<[u8; 80]>,
// }

// impl<Scalar: PrimeField> Circuit<Scalar> for MyCircuit {
//     fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
//         // Compute the values for the bits of the preimage. If we are verifying a proof,
//         // we still need to create the same constraints, so we return an equivalent-size
//         // Vec of None (indicating that the value of each bit is unknown).
//         let bit_values = if let Some(preimage) = self.preimage {
//             preimage
//                 .into_iter()
//                 .map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8))
//                 .flatten()
//                 .map(|b| Some(b))
//                 .collect()
//         } else {
//             vec![None; 80 * 8]
//         };
//         assert_eq!(bit_values.len(), 80 * 8);

//         // Witness the bits of the preimage.
//         let preimage_bits = bit_values
//             .into_iter()
//             .enumerate()
//             // Allocate each bit.
//             .map(|(i, b)| AllocatedBit::alloc(cs.namespace(|| format!("preimage bit {}", i)), b))
//             // Convert the AllocatedBits into Booleans (required for the sha256 gadget).
//             .map(|b| b.map(Boolean::from))
//             .collect::<Result<Vec<_>, _>>()?;

//         // Compute hash = SHA-256d(preimage).
//         let hash = sha256d(cs.namespace(|| "SHA-256d(preimage)"), &preimage_bits)?;

//         // Expose the vector of 32 boolean variables as compact public inputs.
//         multipack::pack_into_inputs(cs.namespace(|| "pack hash"), &hash)
//     }
// }

// #[wasm_bindgen]
// pub fn greet_(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
