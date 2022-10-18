use crate::ProofError;
use sak_crypto::hasher::MiMC;
use sak_crypto::{
    groth16, Groth16VerifyingKey, MultiMillerLoop, Parameters, PreparedVerifyingKey, Proof,
};
use sak_crypto::{Bls12, OsRng, Scalar};
use sak_proof_circuit::CoinProofCircuit2to2;
use sak_proof_types::{NewCoin, OldCoin};

// const CIRCUIT_PARAMS_1TO2: &[u8] = include_bytes!("../../../../prebuild/circuit_params_1to2");
const CIRCUIT_PARAMS_2TO2: &[u8] = include_bytes!("../../../../prebuild/circuit_params_2to2");

pub struct CoinProof;

// pub(crate) fn get_mimc_params_1_to_2(
//     constants: &[Scalar],
// ) -> Result<Parameters<Bls12>, ProofError> {
//     match Parameters::<Bls12>::read(&CIRCUIT_PARAMS_1TO2[..], false) {
//         Ok(p) => Ok(p),
//         Err(err) => {
//             return Err(format!("Error getting circuit params, err: {}", err).into());
//         }
//     }
// }

pub(crate) fn get_mimc_params_2_to_2(
    constants: &[Scalar],
) -> Result<Parameters<Bls12>, ProofError> {
    match Parameters::<Bls12>::read(&CIRCUIT_PARAMS_2TO2[..], false) {
        Ok(p) => Ok(p),
        Err(err) => {
            return Err(format!("Error getting circuit params, err: {}", err).into());
        }
    }
}

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }

    // pub fn get_mimc_params_1_to_2() -> Result<Parameters<Bls12>, ProofError> {
    //     match Parameters::<Bls12>::read(&CIRCUIT_PARAMS_1TO2[..], false) {
    //         Ok(p) => Ok(p),
    //         Err(err) => {
    //             return Err(format!("Error getting circuit params, err: {}", err).into());
    //         }
    //     }
    // }

    // pub fn verify_proof_1_to_2(
    //     proof: Proof<Bls12>,
    //     public_inputs: &[Scalar],
    //     _hasher: &MiMC,
    // ) -> Result<bool, ProofError> {
    //     let de_params = Self::get_mimc_params_1_to_2()?;
    //     let pvk = groth16::prepare_verifying_key(&de_params.vk);

    //     let res = match groth16::verify_proof(&pvk, &proof, public_inputs) {
    //         Ok(_) => true,
    //         Err(err) => {
    //             println!("verify_proof(), err: {}", err);

    //             return Err(format!(
    //                 "verifying error, public_inputs({}): {:?}, err: {}",
    //                 public_inputs.len(),
    //                 public_inputs,
    //                 err
    //             )
    //             .into());
    //         }
    //     };

    //     Ok(res)
    // }

    // pub fn generate_proof_1_to_2(
    //     coin_1_old: OldCoin,
    //     coin_1_new: NewCoin,
    //     coin_2_new: NewCoin,
    // ) -> Result<Proof<Bls12>, ProofError> {
    //     let hasher = MiMC::new();
    //     let constants = hasher.get_mimc_constants().to_vec();
    //     let de_params = Self::get_mimc_params_1_to_2()?;

    //     let c = CoinProofCircuit1to2 {
    //         hasher,
    //         coin_1_old,
    //         coin_1_new,
    //         coin_2_new,
    //         constants,
    //     };

    //     let proof = match groth16::create_random_proof(c, &de_params, &mut OsRng) {
    //         Ok(p) => p,
    //         Err(err) => {
    //             return Err(format!("Failed to generate groth16 proof, err: {}", err).into());
    //         }
    //     };

    //     Ok(proof)
    // }

    pub fn verify_proof_2_to_2(
        proof: Proof<Bls12>,
        public_inputs: &[Scalar],
        hasher: &MiMC,
    ) -> Result<bool, ProofError> {
        let constants = hasher.get_mimc_constants();
        let de_params = get_mimc_params_2_to_2(&constants)?;
        let pvk = groth16::prepare_verifying_key(&de_params.vk);

        let res = match groth16::verify_proof(&pvk, &proof, public_inputs) {
            Ok(_) => true,
            Err(err) => {
                println!("verify_proof(), err: {}", err);

                false
            }
        };

        Ok(res)
    }

    pub fn generate_proof_2_to_2(
        coin_1_old: OldCoin,
        coin_2_old: OldCoin,
        coin_1_new: NewCoin,
        coin_2_new: NewCoin,
    ) -> Result<Proof<Bls12>, ProofError> {
        // println!(
        //     "111, coin_1_old: {:?}, \ncoin_2_old: {:?}\ncoin_1_new: {:?}\ncoin_2_new: {:?}",
        //     coin_1_old, coin_2_old, coin_1_new, coin_2_new,
        // );

        let hasher = MiMC::new();
        let constants = hasher.get_mimc_constants().to_vec();
        let de_params = get_mimc_params_2_to_2(&constants)?;

        let c = CoinProofCircuit2to2 {
            hasher,
            coin_1_old,
            coin_2_old,
            coin_1_new,
            coin_2_new,
            constants,
        };

        let proof = match groth16::create_random_proof(c, &de_params, &mut OsRng) {
            Ok(p) => p,
            Err(err) => {
                return Err(format!("Failed to generate groth16 proof, err: {}", err).into());
            }
        };

        Ok(proof)
    }

    pub fn serialize_pi(pi: &Proof<Bls12>) -> Result<Vec<u8>, ProofError> {
        let mut v = Vec::new();

        pi.write(&mut v)?;

        Ok(v)
    }
}
