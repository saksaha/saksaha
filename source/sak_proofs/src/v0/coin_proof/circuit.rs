use crate::{MerkleTree, CM_TREE_DEPTH};
use pairing::MultiMillerLoop;
use rand::rngs::OsRng;
use sak_crypto::{
    groth16, mimc, AllocatedBit, Bls12, Circuit, ConstraintSystem,
    Groth16VerifyingKey, Parameters, PreparedVerifyingKey, Proof, Scalar,
    SynthesisError,
};
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

pub struct CoinCircuit {
    pub leaf: Option<Scalar>,
    pub auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH],
    pub constants: Vec<Scalar>,
}

impl Circuit<Scalar> for CoinCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut cur = match self.leaf {
            Some(a) => Some(a),
            None => Some(Scalar::default()),
        };

        {
            for (idx, layer) in self.auth_path.iter().enumerate() {
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
                    xr_value = cur;
                } else {
                    xl_value = cur;
                    xr_value = Some(temp.0);
                }

                cur = mimc::mimc_cs(cs, xl_value, xr_value, &self.constants);
            }
        };

        cs.alloc_input(
            || "image",
            || cur.ok_or(SynthesisError::AssignmentMissing),
        )?;

        println!("final circuit public input {:?}", cur);

        Ok(())
    }
}
