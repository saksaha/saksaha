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

        let a_sk_1 = self.a_sk_1.or(Some(Scalar::default()));
        let rho_1 = self.rho_1.or(Some(Scalar::default()));
        let r_1 = self.r_1.or(Some(Scalar::default()));
        let s_1 = self.s_1.or(Some(Scalar::default()));
        let v_1 = self.v_1.or(Some(Scalar::default()));

        let a_sk_2 = self.a_sk_2.or(Some(Scalar::default()));
        let rho_2 = self.rho_2.or(Some(Scalar::default()));
        let r_2 = self.r_2.or(Some(Scalar::default()));
        let s_2 = self.s_2.or(Some(Scalar::default()));
        let v_2 = self.v_2.or(Some(Scalar::default()));

        // rt
        {
            for (idx, layer) in self.auth_path_1.iter().enumerate() {
                println!("idx: {}, layer: {:?}", idx, layer);

                let cs = &mut cs.namespace(|| format!("layer {}", idx));

                let cur_is_right = AllocatedBit::alloc(
                    cs.namespace(|| "cur is right"),
                    layer.as_ref().map(|&(_, d)| d),
                )
                .unwrap();

                let xl_value;
                let xr_value;

                let is_right = cur_is_right.get_value().and_then(|v| {
                    if v {
                        Some(true)
                    } else {
                        Some(false)
                    }
                });

                let temp = match *layer {
                    Some(a) => a,
                    None => (Scalar::default(), false),
                };

                if match is_right {
                    Some(a) => a,
                    None => false,
                } {
                    xl_value = Some(temp.0);
                    xr_value = merkle_rt;
                } else {
                    xl_value = merkle_rt;
                    xr_value = Some(temp.0);
                }

                merkle_rt =
                    mimc::mimc_cs(cs, xl_value, xr_value, &self.constants);
            }
        };

        let (a_pk_1, sn_1, k_1, cm_1) = {
            // pk == PRF(a_sk, 0)
            let a_pk_1: Option<Scalar> =
                self.hasher.prf_cs(cs, Some(Scalar::from(0)), a_sk_1);

            // sn == PRF(a_sk, rho)
            let sn_1: Option<Scalar> = self.hasher.prf_cs(cs, a_sk_1, rho_1);

            // k == COMM(r, PRF(a_pk, rho))
            let k_1_tmp: Option<Scalar> = self.hasher.prf_cs(cs, a_pk_1, rho_1);
            let k_1: Option<Scalar> = self.hasher.comm_cs(cs, r_1, k_1_tmp);

            // cm == COMM(s, PRF(v, k))
            let cm_1_tmp: Option<Scalar> = self.hasher.prf_cs(cs, v_1, k_1);
            let cm_1: Option<Scalar> = self.hasher.comm_cs(cs, s_1, cm_1_tmp);

            (a_pk_1, sn_1, k_1, cm_1)
        };

        let (a_pk_2, sn_2, k_2, cm_2) = {
            // pk == PRF(a_sk, 0)
            let a_pk_2: Option<Scalar> =
                self.hasher.prf_cs(cs, Some(Scalar::from(0)), a_sk_2);

            // sn == PRF(a_sk, rho)
            let sn_2: Option<Scalar> = self.hasher.prf_cs(cs, a_sk_2, rho_2);

            // k == COMM(r, PRF(a_pk, rho))
            let k_2_tmp: Option<Scalar> = self.hasher.prf_cs(cs, a_pk_2, rho_2);
            let k_2: Option<Scalar> = self.hasher.comm_cs(cs, r_2, k_2_tmp);

            // cm == COMM(s, PRF(v, k))
            let cm_2_tmp: Option<Scalar> = self.hasher.prf_cs(cs, v_2, k_2);
            let cm_2: Option<Scalar> = self.hasher.comm_cs(cs, s_2, cm_2_tmp);

            (a_pk_2, sn_2, k_2, cm_2)
        };

        {
            cs.alloc_input(
                || "rt",
                || merkle_rt.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "a_pk_1",
                || a_pk_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_1",
                || sn_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                //
                || "k_1",
                || k_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_1",
                || cm_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "a_pk_2",
                || a_pk_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_2",
                || sn_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                //
                || "k_2",
                || k_2.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_2",
                || cm_2.ok_or(SynthesisError::AssignmentMissing),
            )?;
        }

        println!();
        println!("[+] Final values from test circuit :");
        println!("<1> merkle_rt: {:?}", merkle_rt);
        //
        println!("<2> a_pk_1: {:?}", a_pk_1);
        println!("<3> sn_1: {:?}", sn_1);
        println!("<4> k_1:  {:?}", k_1);
        println!("<5> cm_1: {:?}", cm_1);
        //
        println!("<6> a_pk_2: {:?}", a_pk_2);
        println!("<7> sn_2: {:?}", sn_2);
        println!("<8> k_2:  {:?}", k_2);
        println!("<9> cm_2: {:?}", cm_2);

        Ok(())
    }
}
