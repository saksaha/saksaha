use crate::{MerkleTree, NewCoin, OldCoin, Path, ProofError, CM_TREE_DEPTH};
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

const PARAM_FILE_NAME: &str = "mimc_params_1_to_2";

pub struct CoinProofCircuit1to2 {
    pub hasher: Hasher,

    pub coin_1_old: OldCoin,

    pub coin_1_new: NewCoin,

    pub coin_2_new: NewCoin,

    pub constants: Vec<Scalar>,
}

pub fn get_1_to_2_params(constants: &[Scalar]) -> Parameters<Bls12> {
    let param_path = std::path::Path::new(PARAM_FILE_NAME);
    let is_file_exist = param_path.exists();

    let mut v = vec![];

    if is_file_exist {
        // read
        v = std::fs::read(PARAM_FILE_NAME).unwrap();
    } else {
        // generate and write
        let hasher = Hasher::new();

        let coin_1_old = OldCoin::default();
        let coin_1_new = NewCoin::default();
        let coin_2_new = NewCoin::default();

        let params = {
            let c = CoinProofCircuit1to2 {
                hasher,
                coin_1_old,
                coin_1_new,
                coin_2_new,
                constants: constants.to_vec(),
            };

            groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
                .unwrap()
        };
        // write param to file
        let mut file = File::create(PARAM_FILE_NAME).unwrap();

        params.write(&mut v).unwrap();
        // write origin buf
        file.write_all(&v);
    }

    let de_params = Parameters::<Bls12>::read(&v[..], false).unwrap();
    de_params
}

impl Circuit<Scalar> for CoinProofCircuit1to2 {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let rho_1_old = self.coin_1_old.rho.or(Some(Scalar::default()));
        let addr_pk_1_old = self.coin_1_old.addr_pk.or(Some(Scalar::default()));
        let addr_sk_1_old = self.coin_1_old.addr_sk.or(Some(Scalar::default()));
        let cm_1_old = self.coin_1_old.cm.or(Some(Scalar::default()));
        let r_1_old = self.coin_1_old.r.or(Some(Scalar::default()));
        let s_1_old = self.coin_1_old.s.or(Some(Scalar::default()));
        let v_1_old = self.coin_1_old.v.or(Some(Scalar::default()));

        check_cm_commitments(
            cs,
            cm_1_old,
            addr_pk_1_old,
            rho_1_old,
            r_1_old,
            s_1_old,
            v_1_old,
            &self.hasher,
        );

        let sn_1 = self.hasher.mimc_scalar_cs(cs, addr_sk_1_old, rho_1_old);

        let merkle_rt = climb_up_tree(
            cs,
            cm_1_old,
            &self.coin_1_old.auth_path,
            &self.hasher,
        );

        let addr_pk_1_new = self.coin_1_new.addr_pk.or(Some(Scalar::default()));
        let rho_1_new = self.coin_1_new.rho.or(Some(Scalar::default()));
        let r_1_new = self.coin_1_new.r.or(Some(Scalar::default()));
        let s_1_new = self.coin_1_new.s.or(Some(Scalar::default()));
        let v_1_new = self.coin_1_new.v.or(Some(Scalar::default()));

        let cm_1_new = {
            let k = self.hasher.comm2_scalar_cs(
                cs,
                r_1_new,
                addr_pk_1_new,
                rho_1_new,
            );
            self.hasher.comm2_scalar_cs(cs, s_1_new, v_1_new, k)
        };

        check_cm_commitments(
            cs,
            cm_1_new,
            addr_pk_1_new,
            rho_1_new,
            r_1_new,
            s_1_new,
            v_1_new,
            &self.hasher,
        );

        let addr_pk_2_new = self.coin_2_new.addr_pk.or(Some(Scalar::default()));
        let rho_2_new = self.coin_2_new.rho.or(Some(Scalar::default()));
        let r_2_new = self.coin_2_new.r.or(Some(Scalar::default()));
        let s_2_new = self.coin_2_new.s.or(Some(Scalar::default()));
        let v_2_new = self.coin_2_new.v.or(Some(Scalar::default()));

        let cm_2_new = {
            let k = self.hasher.comm2_scalar_cs(
                cs,
                r_2_new,
                addr_pk_2_new,
                rho_2_new,
            );
            self.hasher.comm2_scalar_cs(cs, s_2_new, v_2_new, k)
        };

        check_cm_commitments(
            cs,
            cm_2_new,
            addr_pk_2_new,
            rho_2_new,
            r_2_new,
            s_2_new,
            v_2_new,
            &self.hasher,
        );

        require_equal_val_summation(cs, v_1_old, v_1_new, v_2_new);

        {
            cs.alloc_input(
                || "merkle_rt",
                || merkle_rt.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_old_1",
                || sn_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_new_1",
                || cm_1_new.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_2_new",
                || cm_2_new.ok_or(SynthesisError::AssignmentMissing),
            )?;
        }

        Ok(())
    }
}

pub fn climb_up_tree<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    leaf: Option<Scalar>,
    auth_path: &[Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
    hasher: &Hasher,
) -> Option<Scalar> {
    let mut curr = leaf;

    for (idx, merkle_node) in auth_path.iter().enumerate() {
        // println!("idx: {}, sibling: {:?}", idx, merkle_node);

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

pub fn check_cm_commitments<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    cm_old: Option<Scalar>,
    addr_pk: Option<Scalar>,
    rho: Option<Scalar>,
    r: Option<Scalar>,
    s: Option<Scalar>,
    v: Option<Scalar>,
    hasher: &Hasher,
) {
    {
        let k = hasher.comm2_scalar_cs(cs, r, addr_pk, rho);

        let cm_computed = hasher.comm2_scalar_cs(cs, s, v, k);

        let cm_1_old = cs
            .alloc(|| "cm", || cm_old.ok_or(SynthesisError::AssignmentMissing))
            .unwrap();

        let cm = cs
            .alloc(
                || "cm_computed",
                || cm_computed.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        cs.enforce(
            || "cm = cm_computed",
            |lc| lc + cm_1_old,
            |lc| lc + CS::one(),
            |lc| lc + cm,
        );
    }
}

pub fn require_equal_val_summation<CS: ConstraintSystem<Scalar>>(
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
