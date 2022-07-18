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

    // old coin (1)
    pub addr_sk_1_old: Option<Scalar>,
    pub rho_1_old: Option<Scalar>,
    pub r_1_old: Option<Scalar>,
    pub s_1_old: Option<Scalar>,
    pub v_1_old: Option<Scalar>,
    pub cm_1_old: Option<Scalar>,

    pub auth_path_1: [Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    pub merkle_rt: Option<Scalar>,

    // new coin 1
    pub addr_sk_1: Option<Scalar>,
    pub rho_1: Option<Scalar>,
    pub r_1: Option<Scalar>,
    pub s_1: Option<Scalar>,
    pub v_1: Option<Scalar>,

    // new coin 2
    pub addr_sk_2: Option<Scalar>,
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
        let mut merkle_rt = self.merkle_rt.or(Some(Scalar::default()));

        let addr_sk_1_old = self.addr_sk_1_old.or(Some(Scalar::default()));
        //

        let addr_sk_1 = self.addr_sk_1.or(Some(Scalar::default()));
        let rho_1 = self.rho_1.or(Some(Scalar::default()));
        let r_1 = self.r_1.or(Some(Scalar::default()));
        let s_1 = self.s_1.or(Some(Scalar::default()));
        let v_1 = self.v_1.or(Some(Scalar::default()));
        let cm_1_old = self.cm_1_old.or(Some(Scalar::default()));

        let a_sk_2 = self.addr_sk_2.or(Some(Scalar::default()));
        let rho_2 = self.rho_2.or(Some(Scalar::default()));
        let r_2 = self.r_2.or(Some(Scalar::default()));
        let s_2 = self.s_2.or(Some(Scalar::default()));
        let v_2 = self.v_2.or(Some(Scalar::default()));

        // sn_1
        let addr_pk_1_old =
            self.hasher.mimc_single_scalar_cs(cs, self.addr_sk_1_old);

        let curr =
            climb_up_tree(cs, cm_1_old, &self.auth_path_1, &self.constants);

        {
            cs.alloc_input(
                || "merkle_rt",
                || curr.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "addr_pk_1_old",
                || addr_pk_1_old.ok_or(SynthesisError::AssignmentMissing),
            )?;
            // cs.alloc_input(
            //     || "a_pk_1",
            //     || a_pk_1.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     || "sn_1",
            //     || sn_1.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     //
            //     || "k_1",
            //     || k_1.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     || "cm_1",
            //     || cm_1.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     || "a_pk_2",
            //     || a_pk_2.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     || "sn_2",
            //     || sn_2.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     //
            //     || "k_2",
            //     || k_2.ok_or(SynthesisError::AssignmentMissing),
            // )?;

            // cs.alloc_input(
            //     || "cm_2",
            //     || cm_2.ok_or(SynthesisError::AssignmentMissing),
            // )?;
        }

        println!();
        println!("[+] Final values from test circuit :");
        println!("<1> merkle_rt: {:?}", merkle_rt);
        //
        // println!("<2> a_pk_1: {:?}", a_pk_1);
        // println!("<3> sn_1: {:?}", sn_1);
        // println!("<4> k_1:  {:?}", k_1);
        // println!("<5> cm_1: {:?}", cm_1);
        // //
        // println!("<6> a_pk_2: {:?}", a_pk_2);
        // println!("<7> sn_2: {:?}", sn_2);
        // println!("<8> k_2:  {:?}", k_2);
        // println!("<9> cm_2: {:?}", cm_2);

        Ok(())
    }
}

fn climb_up_tree<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    leaf: Option<Scalar>,
    auth_path: &[Option<(Scalar, bool)>; TEST_TREE_DEPTH],
    constants: &Vec<Scalar>,
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

        curr = mimc::mimc_cs(cs, xl_value, xr_value, constants);
    }

    return curr;
}
