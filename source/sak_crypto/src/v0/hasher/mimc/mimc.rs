use super::circuit;
use super::mimc;
use crate::CryptoError;
use crate::Scalar;
use crate::ScalarExt;
use bellman::gadgets::boolean::AllocatedBit;
use bellman::gadgets::boolean::Boolean;
use bellman::Circuit;
use bellman::{ConstraintSystem, SynthesisError};
use ff::PrimeField;

pub struct MiMC {
    constants: Vec<Scalar>,
}

impl MiMC {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mimc_constants = circuit::get_mimc_constants();

        Self {
            constants: mimc_constants,
        }
    }

    pub fn get_mimc_constants(&self) -> &Vec<Scalar> {
        return &self.constants;
    }

    pub fn mimc(&self, a: &[u8; 32], b: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let a = ScalarExt::parse_arr(a)?;
        let b = ScalarExt::parse_arr(b)?;
        let h = mimc::mimc(a, b, &self.constants);

        Ok(h)
    }

    pub fn mimc_scalar(&self, a: Scalar, b: Scalar) -> Scalar {
        mimc::mimc(a, b, &self.constants)
    }

    pub fn mimc_single(&self, a: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let a = ScalarExt::parse_arr(a)?;
        let b = Scalar::zero();
        let h = mimc::mimc(a, b, &self.constants);

        Ok(h)
    }

    pub fn mimc_single_scalar(&self, a: Scalar) -> Result<Scalar, CryptoError> {
        let b = Scalar::zero();
        let h = mimc::mimc(a, b, &self.constants);

        Ok(h)
    }

    pub fn mimc_single_scalar_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
    ) -> Option<Scalar> {
        let b = Some(Scalar::zero());
        circuit::mimc_cs(cs, a, b, &self.constants)
    }

    pub fn mimc_scalar_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
        b: Option<Scalar>,
    ) -> Option<Scalar> {
        circuit::mimc_cs(cs, a, b, &self.constants)
    }

    pub fn prf2(&self, a: &[u8; 32], b: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let s = ScalarExt::parse_arr_wide(a, b)?;

        let ret = circuit::mimc_single_arg(s, &self.constants);

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
        self.mimc_scalar_cs(cs, z, x)
    }

    pub fn comm2(&self, a: &[u8; 32], b: &[u8; 32], c: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let a = ScalarExt::parse_arr(a)?;
        let b = ScalarExt::parse_arr(b)?;
        let c = ScalarExt::parse_arr(c)?;

        let r1 = mimc::mimc(b, c, &self.constants);
        let r2 = mimc::mimc(a, r1, &self.constants);

        Ok(r2)
    }

    pub fn comm2_scalar(&self, a: Scalar, b: Scalar, c: Scalar) -> Scalar {
        let r1 = mimc::mimc(b, c, &self.constants);

        let r2 = mimc::mimc(a, r1, &self.constants);

        r2
    }

    pub fn a() {}

    pub fn comm2_scalar_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        a: Option<Scalar>,
        b: Option<Scalar>,
        c: Option<Scalar>,
    ) -> Option<Scalar> {
        let r1 = self.mimc_scalar_cs(cs, b, c);

        let r2 = self.mimc_scalar_cs(cs, a, r1);

        r2
    }

    pub fn comm(&self, r: Scalar, x: Scalar) -> Scalar {
        mimc::mimc(r, x, &self.constants)
    }
}
