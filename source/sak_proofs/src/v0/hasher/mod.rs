use bls12_381::Scalar;

use crate::{get_mimc_constants, mimc};

pub(crate) struct Hasher {
    constants: Vec<Scalar>,
}

impl Hasher {
    pub fn new() -> Hasher {
        let mimc_constants = get_mimc_constants();
        Hasher {
            constants: mimc_constants,
        }
    }

    /// pseudo random function
    pub fn prf(&self, x: Scalar, z: Scalar) -> Scalar {
        mimc(x, z, &self.constants)
    }

    /// pseudo random function for constraint system
    pub fn prf_cs(&self) {}

    /// commitment generating hash function
    pub fn comm(&self) {}

    /// commitment generating hash function for constraint system
    pub fn comm_cs(&self) {}
}
