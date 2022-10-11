use super::utils;
use crate::{CoinProof, ProofError};
use sak_crypto::groth16::{self, Proof};
use sak_crypto::hasher::MiMC;
use sak_crypto::{Bls12, MerkleTree, MerkleTreeSim, OsRng, Scalar, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_logger::debug;
use sak_proof_circuit::{CoinProofCircuit1to2, NewCoin, OldCoin};
use type_extension::U8Array;

pub struct TestContext {
    pub hasher: MiMC,

    // old coin 1
    pub addr_pk_1_old: Scalar,
    pub addr_sk_1_old: Scalar,
    pub r_1_old: Scalar,
    pub s_1_old: Scalar,
    pub rho_1_old: Scalar,
    pub v_1_old: Scalar,
    pub cm_1_old: Scalar,
    pub auth_path_1: [(Scalar, bool); CM_TREE_DEPTH as usize],
    pub merkle_rt: Scalar,
    pub sn_1: Scalar,

    // new coin 1
    pub addr_sk_1: Scalar,
    pub addr_pk_1: Scalar,
    pub r_1: Scalar,
    pub s_1: Scalar,
    pub rho_1: Scalar,
    pub v_1: Scalar,
    pub cm_1: Scalar,

    // new coin 2
    pub addr_sk_2: Scalar,
    pub addr_pk_2: Scalar,
    pub r_2: Scalar,
    pub s_2: Scalar,
    pub rho_2: Scalar,
    pub v_2: Scalar,
    pub cm_2: Scalar,
}

pub fn make_test_context() -> TestContext {
    let hasher = MiMC::new();

    let (addr_pk_1_old, addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old, cm_1_old, sn_1) = {
        let addr_sk = {
            let arr = U8Array::from_int(1);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(2);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(3);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(4);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(100);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

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

    let (addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk = {
            let arr = U8Array::from_int(21);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(22);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(23);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(24);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(30);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let tree_simulator = MerkleTreeSim::init(CM_TREE_DEPTH as u32, vec![cm_1_old]).unwrap();

    let merkle_tree = tree_simulator.merkle_tree;

    let merkle_nodes = tree_simulator.nodes;

    let merkle_rt = *merkle_nodes
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    let auth_path_1 = {
        let v = merkle_tree.generate_auth_paths(0);
        let mut ret = [(Scalar::default(), false); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);

            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            ret[idx] = (merkle_node.clone(), p.direction);
        });

        ret
    };

    TestContext {
        hasher,
        addr_pk_1_old,
        addr_sk_1_old,
        r_1_old,
        s_1_old,
        rho_1_old,
        v_1_old,
        cm_1_old,
        auth_path_1,
        merkle_rt,
        sn_1,
        addr_sk_1,
        addr_pk_1,
        r_1,
        s_1,
        rho_1,
        v_1,
        cm_1,
        addr_sk_2,
        addr_pk_2,
        r_2,
        s_2,
        rho_2,
        v_2,
        cm_2,
    }
}

fn make_proof(
    coin_1_old: OldCoin,
    coin_1_new: NewCoin,
    coin_2_new: NewCoin,
) -> Result<Proof<Bls12>, ProofError> {
    let hasher = MiMC::new();
    let constants = hasher.get_mimc_constants().to_vec();
    let de_params = CoinProof::get_mimc_params_1_to_2()?;

    let c = CoinProofCircuit1to2 {
        hasher,
        coin_1_old,
        coin_1_new,
        coin_2_new,
        constants,
    };

    let proof = match groth16::create_random_proof(c, &de_params, &mut OsRng) {
        Ok(p) => p,
        Err(err) => {
            return Err(format!("Failed to generate groth16 proof, err: {}", err).into());
        }
    };

    Ok(proof)
}

fn verify_proof(
    proof: Proof<Bls12>,
    public_inputs: &[Scalar],
    _hasher: &MiMC,
) -> Result<bool, ProofError> {
    let de_params = CoinProof::get_mimc_params_1_to_2()?;
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

    match groth16::verify_proof(&pvk, &proof, public_inputs) {
        Ok(_) => {
            debug!("Verify success!, public input: {:?}", public_inputs);
            Ok(true)
        }
        Err(err) => Err(format!("Verify proof failed, err: {}", err).into()),
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default_1_to_2() {
    utils::init_test_log();

    let test_context = make_test_context();

    let coin_1_old = OldCoin {
        addr_pk: Some(test_context.addr_pk_1_old),
        addr_sk: Some(test_context.addr_sk_1_old),
        rho: Some(test_context.rho_1_old),
        r: Some(test_context.r_1_old),
        s: Some(test_context.s_1_old),
        v: Some(test_context.v_1_old),
        cm: Some(test_context.cm_1_old),
        auth_path: test_context.auth_path_1.map(|e| Some(e)),
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(test_context.addr_pk_1),
        rho: Some(test_context.rho_1),
        r: Some(test_context.r_1),
        s: Some(test_context.s_1),
        v: Some(test_context.v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(test_context.addr_pk_2),
        rho: Some(test_context.rho_2),
        r: Some(test_context.r_2),
        s: Some(test_context.s_2),
        v: Some(test_context.v_2),
    };

    let proof = make_proof(coin_1_old, coin_1_new, coin_2_new).unwrap();

    let public_inputs: Vec<Scalar> = vec![
        test_context.merkle_rt,
        test_context.sn_1,
        test_context.cm_1,
        test_context.cm_2,
    ];

    verify_proof(proof, &public_inputs, &test_context.hasher).unwrap();
}
