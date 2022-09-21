use crate::{CircuitError, Hasher, NewCoin, OldCoin};
use bellman::gadgets::boolean::AllocatedBit;
use bellman::groth16::{self, Parameters};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use sak_crypto::{Bls12, OsRng, Scalar};
use sak_dist_ledger_meta::{CM_TREE_DEPTH, GAS};
use type_extension::U8Array;

pub struct CoinProofCircuit2to2 {
    pub hasher: Hasher,

    pub coin_1_old: OldCoin,

    pub coin_2_old: OldCoin,

    pub coin_1_new: NewCoin,

    pub coin_2_new: NewCoin,

    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for CoinProofCircuit2to2 {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let old_coins = vec![self.coin_1_old, self.coin_2_old];

        let mut sns: Vec<Option<Scalar>> = Vec::default();

        let mut merkle_rts: Vec<Option<Scalar>> = Vec::default();

        let mut old_values: Vec<Option<Scalar>> = Vec::default();

        for old_coin in old_coins {
            //
            let rho_old = old_coin.rho.or(Some(Scalar::default()));
            let addr_pk_old = old_coin.addr_pk.or(Some(Scalar::default()));
            let addr_sk_old = old_coin.addr_sk.or(Some(Scalar::default()));
            let cm_old = old_coin.cm.or(Some(Scalar::default()));
            let r_old = old_coin.r.or(Some(Scalar::default()));
            let s_old = old_coin.s.or(Some(Scalar::default()));
            let v_old = old_coin.v.or(Some(Scalar::default()));

            old_values.push(v_old);

            check_cm_commitments(
                cs,
                cm_old,
                addr_pk_old,
                rho_old,
                r_old,
                s_old,
                v_old,
                &self.hasher,
            );

            let sn = self.hasher.mimc_scalar_cs(cs, addr_sk_old, rho_old);

            sns.push(sn);

            let merkle_rt = climb_up_tree(cs, cm_old, &old_coin.auth_path, &self.hasher);

            merkle_rts.push(merkle_rt);
        }

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

        require_equal_val_summation_2_to_2(cs, old_values, v_1_new, v_2_new);

        {
            cs.alloc_input(
                || "merkle_rt_1",
                || merkle_rts[0].ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "merkle_rt_2",
                || merkle_rts[1].ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_1_old",
                || sns[0].ok_or(SynthesisError::AssignmentMissing),
            )?;

            cs.alloc_input(
                || "sn_2_old",
                || sns[1].ok_or(SynthesisError::AssignmentMissing),
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

        let is_right =
            cur_is_right
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

// pub fn climb_up_tree_2_to_2<CS: ConstraintSystem<Scalar>>(
//     cs: &mut CS,
//     leaf_1: Option<Scalar>,
//     // leaf_2: Option<Scalar>,
//     auth_path_1: &[Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
//     // auth_path_2: &[Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
//     hasher: &Hasher,
// ) -> Option<Scalar> {
//     let leaves = [leaf_1, leaf_2];
//     let auth_paths = [auth_path_1, auth_path_2];

//     let mut curr = Scalar::default();

//     for leaf in leaves {
//         let mut curr = leaf;

//         for (idx, merkle_node) in auth_path_1.iter().enumerate() {
//             // println!("idx: {}, sibling: {:?}", idx, merkle_node);

//             let cs = &mut cs.namespace(|| format!("height {}", idx));

//             let cur_is_right = AllocatedBit::alloc(
//                 cs.namespace(|| "cur is right"),
//                 merkle_node.as_ref().map(|&(_, d)| d),
//             )
//             .expect("cur_is_right");

//             let xl_value;
//             let xr_value;

//             let is_right = cur_is_right.get_value().and_then(|v| {
//                 if v {
//                     Some(true)
//                 } else {
//                     Some(false)
//                 }
//             });

//             let temp = match *merkle_node {
//                 Some(a) => a,
//                 None => (Scalar::default(), false),
//             };

//             if match is_right {
//                 Some(a) => a,
//                 None => false,
//             } {
//                 xl_value = Some(temp.0);
//                 xr_value = curr;
//             } else {
//                 xl_value = curr;
//                 xr_value = Some(temp.0);
//             }

//             curr = hasher.mimc_scalar_cs(cs, xl_value, xr_value);
//         }
//     }

//     // let mut curr_2 = leaf_2;

//     // for (idx, merkle_node) in auth_path_2.iter().enumerate() {
//     //     // println!("idx: {}, sibling: {:?}", idx, merkle_node);

//     //     let cs = &mut cs.namespace(|| format!("height {}", idx));

//     //     let cur_is_right = AllocatedBit::alloc(
//     //         cs.namespace(|| "cur is right"),
//     //         merkle_node.as_ref().map(|&(_, d)| d),
//     //     )
//     //     .expect("cur_is_right");

//     //     let xl_value;
//     //     let xr_value;

//     //     let is_right = cur_is_right.get_value().and_then(|v| {
//     //         if v {
//     //             Some(true)
//     //         } else {
//     //             Some(false)
//     //         }
//     //     });

//     //     let temp = match *merkle_node {
//     //         Some(a) => a,
//     //         None => (Scalar::default(), false),
//     //     };

//     //     if match is_right {
//     //         Some(a) => a,
//     //         None => false,
//     //     } {
//     //         xl_value = Some(temp.0);
//     //         xr_value = curr_2;
//     //     } else {
//     //         xl_value = curr_2;
//     //         xr_value = Some(temp.0);
//     //     }

//     //     curr_2 = hasher.mimc_scalar_cs(cs, xl_value, xr_value);
//     // }
//     Some(curr)
// }

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

        let cm_old = cs
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
            |lc| lc + cm_old,
            |lc| lc + CS::one(),
            |lc| lc + cm,
        );
    }
}

pub fn require_equal_val_summation_2_to_2<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    old_values: Vec<Option<Scalar>>,
    v_new_1: Option<Scalar>,
    v_new_2: Option<Scalar>,
) {
    let gas = Some(Scalar::from_bytes(&U8Array::from_int(GAS)).unwrap());

    let v_gas = cs
        .alloc(|| "v_gas", || gas.ok_or(SynthesisError::AssignmentMissing))
        .unwrap();

    let v_1_old = cs
        .alloc(
            || "v_old_1",
            || old_values[0].ok_or(SynthesisError::AssignmentMissing),
        )
        .unwrap();

    let v_2_old = cs
        .alloc(
            || "v_old_2",
            || old_values[1].ok_or(SynthesisError::AssignmentMissing),
        )
        .unwrap();

    {
        let v_1_new = cs
            .alloc(
                || "preimage v_1",
                || v_new_1.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        let v_2_new = cs
            .alloc(
                || "preimage v_2",
                || v_new_2.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap();

        cs.enforce(
            || "v_1_old + v_2_old = v_1 + v_2 + v_gas",
            |lc| lc + v_1_new + v_2_new + v_gas,
            |lc| lc + CS::one(),
            |lc| lc + v_1_old + v_2_old,
        );
    };
}
