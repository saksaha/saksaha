use crate::{MerkleTree, Path, ProofError};
use rand::rngs::OsRng;
use rand::RngCore;
use sak_crypto::{
    groth16, AllocatedBit, Circuit, ConstraintSystem, Proof, SynthesisError,
};
use sak_crypto::{mimc, Parameters};
use sak_crypto::{Bls12, Hasher, Scalar};
use sak_types::U8Array;
use std::fs::File;
use std::io::Write;

const TEST_TREE_DEPTH: usize = 3;

pub(crate) struct CoinProofCircuit1to2 {
    pub hasher: Hasher,

    // old coin 1
    pub addr_pk_1_old: Option<Scalar>,
    pub addr_sk_1_old: Option<Scalar>,
    pub rho_1_old: Option<Scalar>,
    pub r_1_old: Option<Scalar>,
    pub s_1_old: Option<Scalar>,
    pub v_1_old: Option<Scalar>,
    pub cm_1_old: Option<Scalar>,

    pub auth_path_1: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    // pub merkle_rt: Option<Scalar>,

    // new coin 1
    pub addr_pk_1: Option<Scalar>,
    pub rho_1: Option<Scalar>,
    pub r_1: Option<Scalar>,
    pub s_1: Option<Scalar>,
    pub v_1: Option<Scalar>,

    // new coin 2
    pub addr_pk_2: Option<Scalar>,
    pub rho_2: Option<Scalar>,
    pub r_2: Option<Scalar>,
    pub s_2: Option<Scalar>,
    pub v_2: Option<Scalar>,

    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for CoinProofCircuit1to2 {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let rho_1_old = self.rho_1_old.or(Some(Scalar::default()));
        let addr_pk_1_old = self.addr_pk_1_old.or(Some(Scalar::default()));
        let addr_sk_1_old = self.addr_sk_1_old.or(Some(Scalar::default()));
        let cm_1_old = self.cm_1_old.or(Some(Scalar::default()));
        let r_1_old = self.r_1_old.or(Some(Scalar::default()));
        let s_1_old = self.s_1_old.or(Some(Scalar::default()));
        let v_1_old = self.v_1_old.or(Some(Scalar::default()));

        {
            let k = self.hasher.comm2_scalar_cs(
                cs,
                r_1_old,
                addr_pk_1_old,
                rho_1_old,
            );

            let cm = self.hasher.comm2_scalar_cs(cs, s_1_old, v_1_old, k);

            println!("123 cm: {:?}", cm);
            println!("123 cm_1_old: {:?}", cm_1_old);

            let cm_1_old = cs
                .alloc(
                    || "cm_1_old",
                    || cm_1_old.ok_or(SynthesisError::AssignmentMissing),
                )
                .unwrap();

            let cm = cs
                .alloc(|| "cm", || cm.ok_or(SynthesisError::AssignmentMissing))
                .unwrap();

            cs.enforce(
                || "cm_1_old = cm",
                |lc| lc + cm_1_old,
                |lc| lc + CS::one(),
                |lc| lc + cm,
            );
        }

        let sn_1 = self.hasher.mimc_scalar_cs(cs, addr_sk_1_old, rho_1_old);

        let merkle_rt = climb_up_tree(
            cs,
            cm_1_old,
            &self.auth_path_1,
            // &self.constants,
            &self.hasher,
        );

        let addr_pk_1 = self.addr_pk_1.or(Some(Scalar::default()));
        let rho_1 = self.rho_1.or(Some(Scalar::default()));
        let r_1 = self.r_1.or(Some(Scalar::default()));
        let s_1 = self.s_1.or(Some(Scalar::default()));
        let v_1 = self.v_1.or(Some(Scalar::default()));

        let cm_1 = {
            let k = self.hasher.comm2_scalar_cs(cs, r_1, addr_pk_1, rho_1);
            self.hasher.comm2_scalar_cs(cs, s_1, v_1, k)
        };

        let addr_pk_2 = self.addr_pk_2.or(Some(Scalar::default()));
        let rho_2 = self.rho_2.or(Some(Scalar::default()));
        let r_2 = self.r_2.or(Some(Scalar::default()));
        let s_2 = self.s_2.or(Some(Scalar::default()));
        let v_2 = self.v_2.or(Some(Scalar::default()));

        let cm_2 = {
            let k = self.hasher.comm2_scalar_cs(cs, r_2, addr_pk_2, rho_2);
            self.hasher.comm2_scalar_cs(cs, s_2, v_2, k)
        };

        require_equal_val_summation(cs, v_1_old, v_1, v_2);

        {
            cs.alloc_input(
                || "merkle_rt",
                || merkle_rt.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_1",
                || sn_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_1",
                || cm_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_2",
                || cm_2.ok_or(SynthesisError::AssignmentMissing),
            )?;
        }

        println!();
        println!("[+] Final values from test circuit :");

        Ok(())
    }
}

fn climb_up_tree<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    leaf: Option<Scalar>,
    auth_path: &[Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    hasher: &Hasher,
) -> Option<Scalar> {
    let mut curr = leaf;

    for (idx, merkle_node) in auth_path.iter().enumerate() {
        println!("idx: {}, sibling: {:?}", idx, merkle_node);

        let cs = &mut cs.namespace(|| format!("height {}", idx));

        let cur_is_right = AllocatedBit::alloc(
            cs.namespace(|| "cur is right"),
            merkle_node.as_ref().map(|&(_, d)| d),
        )
        .expect("cur_is_right");

        let xl_value;
        let xr_value;

        let is_right = cur_is_right.get_value().and_then(|v| {
            if v {
                Some(true)
            } else {
                Some(false)
            }
        });

        let temp = match *merkle_node {
            Some(a) => a,
            None => (Scalar::default(), false),
        };

        if match is_right {
            Some(a) => a,
            None => false,
        } {
            xl_value = Some(temp.0);
            xr_value = curr;
        } else {
            xl_value = curr;
            xr_value = Some(temp.0);
        }

        curr = hasher.mimc_scalar_cs(cs, xl_value, xr_value);
    }

    return curr;
}

fn require_equal_val_summation<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    v_old: Option<Scalar>,
    v_1: Option<Scalar>,
    v_2: Option<Scalar>,
) {
    let v_old = cs
        .alloc(
            || "v_old",
            || v_old.ok_or(SynthesisError::AssignmentMissing),
        )
        .unwrap();

    {
        let v_1_new = cs
            .alloc(
                || "preimage v_1",
                || v_1.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        let v_2_new = cs
            .alloc(
                || "preimage v_2",
                || v_2.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        cs.enforce(
            || "tmp = v_1 + v_2",
            |lc| lc + v_1_new + v_2_new,
            |lc| lc + CS::one(),
            |lc| lc + v_old,
        );
    };
}
