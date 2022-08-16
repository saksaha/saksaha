use crate::get_mimc_params_1_to_2;
use pairing::MultiMillerLoop;
use sak_crypto::{
    groth16, Bls12, Groth16VerifyingKey, Hasher, PreparedVerifyingKey, Proof,
    Scalar,
};

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }
}

pub fn verify_proof_1_to_2(
    proof: Proof<Bls12>,
    public_inputs: &[Scalar],
    hasher: &Hasher,
) -> bool {
    let constants = hasher.get_mimc_constants();
    let de_params = get_mimc_params_1_to_2(&constants);
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

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
