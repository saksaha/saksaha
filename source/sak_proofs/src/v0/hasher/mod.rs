use bellman::{ConstraintSystem, SynthesisError};
use bls12_381::Scalar;

use crate::{get_mimc_constants, mimc, MIMC_ROUNDS};

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
    pub fn prf_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        mut xl_value: Option<Scalar>,
        mut xr_value: Option<Scalar>,
    ) -> Option<Scalar> {
        // Allocate the first component of the preimage.
        let mut xl = cs
            .alloc(
                || "preimage xl",
                || xl_value.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        // Allocate the second component of the preimage.
        let mut xr = cs
            .alloc(
                || "preimage xr",
                || xr_value.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        for i in 0..MIMC_ROUNDS {
            // xL, xR := xR + (xL + Ci)^3, xL
            let cs = &mut cs.namespace(|| format!("round {}", i));

            // tmp = (xL + Ci)^2
            let tmp_value = xl_value.map(|mut e| {
                e += &self.constants[i];
                e.square()
            });

            let tmp = cs
                .alloc(
                    || "tmp",
                    || tmp_value.ok_or(SynthesisError::AssignmentMissing),
                )
                .unwrap();

            cs.enforce(
                || "tmp = (xL + Ci)^2",
                |lc| lc + xl + (self.constants[i], CS::one()),
                |lc| lc + xl + (self.constants[i], CS::one()),
                |lc| lc + tmp,
            );

            // new_xL = xR + (xL + Ci)^3
            // new_xL = xR + tmp * (xL + Ci)
            // new_xL - xR = tmp * (xL + Ci)
            let new_xl_value = xl_value.map(|mut e| {
                e += &self.constants[i];
                e *= &tmp_value.unwrap();
                e += &xr_value.unwrap();
                e
            });

            let new_xl = cs
                .alloc(
                    || "new_xl",
                    || new_xl_value.ok_or(SynthesisError::AssignmentMissing),
                )
                .unwrap();

            cs.enforce(
                || "new_xL = xR + (xL + Ci)^3",
                |lc| lc + tmp,
                |lc| lc + xl + (self.constants[i], CS::one()),
                |lc| lc + new_xl - xr,
            );

            // xR = xL
            xr = xl;
            xr_value = xl_value;

            // xL = new_xL
            xl = new_xl;
            xl_value = new_xl_value;
        }

        xl_value
    }

    /// commitment generating hash function
    pub fn comm(&self) {}

    /// commitment generating hash function for constraint system
    pub fn comm_cs(&self) {}
}
