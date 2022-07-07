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

    fn mimc_cs_scalar<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        mut a: Option<Scalar>,
        mut b: Option<Scalar>,
    ) -> Option<Scalar> {
        // Allocate the first component of the preimage.
        let mut a_cs = cs
            .alloc(
                || "preimage a_cs",
                || a.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        // Allocate the second component of the preimage.
        let mut b_cs = cs
            .alloc(
                || "preimage b_cs",
                || b.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        for i in 0..MIMC_ROUNDS {
            // a_cs, b_cs := b_cs + (a_cs + Ci)^3, a_cs
            let cs = &mut cs.namespace(|| format!("round {}", i));

            // tmp = (a_cs + Ci)^2
            let tmp_value = a.map(|mut e| {
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
                || "tmp = (a_cs + Ci)^2",
                |lc| lc + a_cs + (self.constants[i], CS::one()),
                |lc| lc + a_cs + (self.constants[i], CS::one()),
                |lc| lc + tmp,
            );

            // new_a_cs = b_cs + (a_cs + Ci)^3
            // new_a_cs = b_cs + tmp * (a_cs + Ci)
            // new_a_cs - b_cs = tmp * (a_cs + Ci)
            let new_a = a.map(|mut e| {
                e += &self.constants[i];
                e *= &tmp_value.unwrap();
                e += &b.unwrap();
                e
            });

            let new_a_cs = cs
                .alloc(
                    || "new_a_cs",
                    || new_a.ok_or(SynthesisError::AssignmentMissing),
                )
                .unwrap();

            cs.enforce(
                || "new_a_cs = b_cs + (a_cs + Ci)^3",
                |lc| lc + tmp,
                |lc| lc + a_cs + (self.constants[i], CS::one()),
                |lc| lc + new_a_cs - b_cs,
            );

            // b_cs = a_cs
            b_cs = a_cs;
            b = a;

            // a_cs = new_a_cs
            a_cs = new_a_cs;
            a = new_a;
        }

        a
    }

    /// pseudo random function
    pub fn prf(&self, z: Scalar, x: Scalar) -> Scalar {
        mimc(z, x, &self.constants)
    }

    /// pseudo random function for constraint system
    pub fn prf_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        mut z: Option<Scalar>,
        mut x: Option<Scalar>,
    ) -> Option<Scalar> {
        self.mimc_cs_scalar(cs, z, x)
    }

    /// commitment generating hash function
    pub fn comm(&self, r: Scalar, x: Scalar) -> Scalar {
        mimc(r, x, &self.constants)
    }

    /// commitment generating hash function for constraint system
    pub fn comm_cs<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        mut r: Option<Scalar>,
        mut x: Option<Scalar>,
    ) -> Option<Scalar> {
        self.mimc_cs_scalar(cs, r, x)
    }
}
