use crate::{CoinProof, ProofError};
use sak_crypto::groth16::{self, Parameters, Proof};
use sak_crypto::hasher::MiMC;
use sak_crypto::{Bls12, MerkleTree, OsRng, Scalar, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_proof_circuit::{CoinProofCircuit1to2, NewCoin, OldCoin};
use std::collections::HashMap;
use type_extension::U8Array;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_123() {
    println!("poower");

    let hasher = MiMC::new();

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
        let addr_sk = {
            let arr = U8Array::from_int(11);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(12);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(13);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(14);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(60);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };
}
