use crate::mimc;
use crate::CryptoError;
use crate::Scalar;
use crate::ScalarExt;
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

    pub fn mimc(
        &self,
        a: &[u8; 32],
        b: &[u8; 32],
    ) -> Result<Scalar, CryptoError> {
        let a = ScalarExt::parse_arr(a)?;
        let b = ScalarExt::parse_arr(b)?;
        let h = mimc::mimc(a, b, &self.constants);

        Ok(h)
    }

    pub fn mimc_single(&self, a: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let a = ScalarExt::parse_arr(a)?;
        let b = Scalar::zero();
        let h = mimc::mimc(a, b, &self.constants);

        Ok(h)
    }

    // pub fn mimc(&self, xl: Scalar, xr: Scalar) -> Scalar {
    //     mimc::mimc(xl, xr, &self.constants)
    // }

    fn mimc_cs_scalar<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
        b: Option<Scalar>,
    ) -> Option<Scalar> {
        mimc::mimc_cs(cs, a, b, &self.constants)
    }

    pub fn prf2(
        &self,
        a: &[u8; 32],
        b: &[u8; 32],
    ) -> Result<Scalar, CryptoError> {
        let s = ScalarExt::parse_arr_wide(a, b)?;

        let ret = mimc::mimc_single_arg(s, &self.constants);

        Ok(ret)
    }

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

    pub fn comm2(
        &self,
        a: &[u8; 32],
        b: &[u8; 32],
        c: &[u8; 32],
    ) -> Result<Scalar, CryptoError> {
        let s1 = ScalarExt::parse_arr(a)?;

        let s2 = ScalarExt::parse_arr_wide(b, c)?;

        let ret = mimc::mimc(s1, s2, &self.constants);

        Ok(ret)
    }

    // pub fn comm2(&self, a: Scalar, b: Scalar, c: Scalar) -> Scalar {
    //     let r1 = mimc::mimc(b, c, &self.constants);

    //     let r2 = mimc::mimc(a, r1, &self.constants);

    //     r2
    // }

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
