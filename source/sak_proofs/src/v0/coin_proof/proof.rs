use crate::{
    get_mimc_params_1_to_2, CoinProofCircuit1to2, Hasher, NewCoin, OldCoin,
    ProofError,
};
use bellman::groth16;
use pairing::MultiMillerLoop;
use sak_crypto::{
    Bls12, Groth16VerifyingKey, OsRng, PreparedVerifyingKey, Proof, Scalar,
};

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }

    pub fn verify_proof_1_to_2(
        proof: Proof<Bls12>,
        public_inputs: &[Scalar],
        hasher: &Hasher,
    ) -> Result<bool, ProofError> {
        let constants = hasher.get_mimc_constants();
        let de_params = get_mimc_params_1_to_2(&constants)?;
        let pvk = groth16::prepare_verifying_key(&de_params.vk);

        let res = match groth16::verify_proof(&pvk, &proof, public_inputs) {
            Ok(_) => {
                println!("verify success!");

                true
            }
            Err(err) => {
                println!("verify_proof(), err: {}", err);

                false
            }
        };

        Ok(res)
    }

    pub fn generate_proof_1_to_2(
        coin_1_old: OldCoin,
        coin_1_new: NewCoin,
        coin_2_new: NewCoin,
    ) -> Result<Proof<Bls12>, ProofError> {
        let hasher = Hasher::new();
        let constants = hasher.get_mimc_constants().to_vec();

        let de_params = get_mimc_params_1_to_2(&constants)?;

        let c = CoinProofCircuit1to2 {
            hasher,
            coin_1_old,
            coin_1_new,
            coin_2_new,
            constants,
        };

        let proof =
            match groth16::create_random_proof(c, &de_params, &mut OsRng) {
                Ok(p) => p,
                Err(err) => {
                    return Err(format!(
                        "Failed to generate groth16 proof, err: {}",
                        err
                    )
                    .into());
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
