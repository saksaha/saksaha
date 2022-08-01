use pairing::MultiMillerLoop;
use sak_crypto::{groth16, Groth16VerifyingKey, PreparedVerifyingKey};

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }
}
