use crate::mimc;
use crate::Scalar;
use bellman::{ConstraintSystem, SynthesisError};

pub struct Hasher {
    constants: Vec<Scalar>,
}

impl Hasher {
    #[allow(dead_code)]
    pub fn new() -> Hasher {
        let mimc_constants = mimc::get_mimc_constants();

        Hasher {
            constants: mimc_constants,
        }
    }

    pub fn mimc(&self, xl: Scalar, xr: Scalar) -> Scalar {
        mimc::mimc(xl, xr, &self.constants)
    }

    fn mimc_cs_scalar<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
        b: Option<Scalar>,
    ) -> Option<Scalar> {
        mimc::mimc_cs(cs, a, b, &self.constants)
    }

    /// pseudo random function
    #[allow(dead_code)]
    pub fn prf(&self, z: Scalar, x: Scalar) -> Scalar {
        mimc::mimc(z, x, &self.constants)
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

    pub fn comm2(&self, a: Scalar, b: Scalar, c: Scalar) -> Scalar {
        let r1 = mimc::mimc(b, c, &self.constants);

        let r2 = mimc::mimc(a, r1, &self.constants);

        r2
    }

    pub fn comm(&self, r: Scalar, x: Scalar) -> Scalar {
        mimc::mimc(r, x, &self.constants)
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
