use crate::{CircuitError, NewCoin, OldCoin};
use bellman::gadgets::boolean::AllocatedBit;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use sak_crypto::hasher::MiMC;
use sak_crypto::Scalar;
use sak_dist_ledger_config::{CM_TREE_DEPTH, GAS};
use type_extension::U8Array;

pub struct CoinProofCircuit1to2 {
    pub hasher: MiMC,

    pub coin_1_old: OldCoin,

    pub coin_1_new: NewCoin,

    pub coin_2_new: NewCoin,

    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for CoinProofCircuit1to2 {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
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

        let merkle_rt = climb_up_tree(cs, cm_1_old, &self.coin_1_old.auth_path, &self.hasher);

        let addr_pk_1_new = self.coin_1_new.addr_pk.or(Some(Scalar::default()));
        let rho_1_new = self.coin_1_new.rho.or(Some(Scalar::default()));
        let r_1_new = self.coin_1_new.r.or(Some(Scalar::default()));
        let s_1_new = self.coin_1_new.s.or(Some(Scalar::default()));
        let v_1_new = self.coin_1_new.v.or(Some(Scalar::default()));

        let cm_1_new = {
            let k = self
                .hasher
                .comm2_scalar_cs(cs, r_1_new, addr_pk_1_new, rho_1_new);
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
            let k = self
                .hasher
                .comm2_scalar_cs(cs, r_2_new, addr_pk_2_new, rho_2_new);
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
                || "sn_1_old",
                || sn_1.ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "cm_1_new",
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
    hasher: &MiMC,
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

        let is_right = cur_is_right
            .get_value()
            .and_then(|v| if v { Some(true) } else { Some(false) });

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
    hasher: &MiMC,
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
    let gas = Some(Scalar::from_bytes(&U8Array::from_int(GAS)).unwrap());

    let v_gas = cs
        .alloc(|| "v_gas", || gas.ok_or(SynthesisError::AssignmentMissing))
        .unwrap();

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
            || "tmp = v_1 + v_2 + v_gas = v_old",
            |lc| lc + v_1_new + v_2_new + v_gas,
            |lc| lc + CS::one(),
            |lc| lc + v_old,
        );
    };
}
