use crate::{get_mimc_constants, mimc, mimc_cs};
use bellman::{ConstraintSystem, SynthesisError};
use bls12_381::Scalar;

pub(crate) struct Hasher {
    constants: Vec<Scalar>,
}

impl Hasher {
    #[allow(dead_code)]
    pub fn new() -> Hasher {
        let mimc_constants = get_mimc_constants();

        Hasher {
            constants: mimc_constants,
        }
    }

    pub fn mimc2(xl: impl AsRef<[u8]>, xr: impl AsRef<[u8]>) -> Scalar {
        let a = Scalar::from(xl);
        // let xl = Scalar::from_bytes(&xl);
        // for c in constants {
        //     let mut tmp1 = xl;
        //     tmp1.add_assign(c);
        //     let mut tmp2 = tmp1.square();
        //     tmp2.mul_assign(&tmp1);
        //     tmp2.add_assign(&xr);
        //     xr = xl;
        //     xl = tmp2;
        // }

        // xl
        Scalar::from(0)
    }

    #[allow(dead_code)]
    fn mimc_cs_scalar<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
        b: Option<Scalar>,
    ) -> Option<Scalar> {
        mimc_cs(cs, a, b, &self.constants)
    }

    /// pseudo random function
    #[allow(dead_code)]
    pub fn prf(&self, z: Scalar, x: Scalar) -> Scalar {
        mimc(z, x, &self.constants)
    }

    /// pseudo random function for constraint system
    #[allow(dead_code)]
    pub fn prf_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        z: Option<Scalar>,
        x: Option<Scalar>,
    ) -> Option<Scalar> {
        self.mimc_cs_scalar(cs, z, x)
    }

    /// commitment generating hash function
    #[allow(dead_code)]
    pub fn comm(&self, r: Scalar, x: Scalar) -> Scalar {
        mimc(r, x, &self.constants)
    }

    /// commitment generating hash function for constraint system
    #[allow(dead_code)]
    pub fn comm_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        r: Option<Scalar>,
        x: Option<Scalar>,
    ) -> Option<Scalar> {
        self.mimc_cs_scalar(cs, r, x)
    }
}
