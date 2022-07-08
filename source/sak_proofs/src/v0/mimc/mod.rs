mod constants;

use bellman::{ConstraintSystem, SynthesisError};
use bls12_381::Scalar;
use ff::PrimeField;

use self::constants::ROUND_CONSTANTS;

pub const MIMC_ROUNDS: usize = 322;

// pub struct MiMC;

// impl MiMC {
pub fn get_mimc_constants() -> Vec<Scalar> {
    let constants = (0..322)
        .map(|idx| Scalar::from_bytes(&ROUND_CONSTANTS[idx]).unwrap())
        .collect::<Vec<_>>();
    constants
}

pub fn mimc_single_arg<S: PrimeField>(xl: S, constants: &[S]) -> S {
    mimc(xl, S::zero(), constants)
}

pub fn mimc<S: PrimeField>(mut xl: S, mut xr: S, constants: &[S]) -> S {
    for c in constants {
        let mut tmp1 = xl;
        tmp1.add_assign(c);
        let mut tmp2 = tmp1.square();
        tmp2.mul_assign(&tmp1);
        tmp2.add_assign(&xr);
        xr = xl;
        xl = tmp2;
    }

    xl
}

pub fn mimc_cs<S: PrimeField, CS: ConstraintSystem<S>>(
    cs: &mut CS,
    mut xl_value: Option<S>,
    mut xr_value: Option<S>,
    round_constants: &[S],
) -> Option<S> {
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
            e.add_assign(&round_constants[i]);
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
            |lc| lc + xl + (round_constants[i], CS::one()),
            |lc| lc + xl + (round_constants[i], CS::one()),
            |lc| lc + tmp,
        );

        // new_xL = xR + (xL + Ci)^3
        // new_xL = xR + tmp * (xL + Ci)
        // new_xL - xR = tmp * (xL + Ci)
        let new_xl_value = xl_value.map(|mut e| {
            e.add_assign(&round_constants[i]);
            e.mul_assign(&tmp_value.unwrap());
            e.add_assign(&xr_value.unwrap());
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
            |lc| lc + xl + (round_constants[i], CS::one()),
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
