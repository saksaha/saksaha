use crate::{CoinProof, ProofError};
use sak_crypto::hasher::MiMC;
use sak_crypto::{Bls12, MerkleTreeSim, Proof, Scalar, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_proof_types::{NewCoin, OldCoin};
use std::os::raw::c_char;
use type_extension::U8Array;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

pub fn pi_gen_1_depth_32() -> String {
    let test_context = make_test_context_2_to_2_depth_32();

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

    let coin_2_old = OldCoin {
        addr_pk: Some(test_context.addr_pk_2_old),
        addr_sk: Some(test_context.addr_sk_2_old),
        rho: Some(test_context.rho_2_old),
        r: Some(test_context.r_2_old),
        s: Some(test_context.s_2_old),
        v: Some(test_context.v_2_old),
        cm: Some(test_context.cm_2_old),
        auth_path: test_context.auth_path_2.map(|e| Some(e)),
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

    let proof = CoinProof::generate_proof_2_to_2(coin_1_old, coin_2_old, coin_1_new, coin_2_new)
        .expect("proof should be created");

    let mut pi_ser = Vec::new();
    match proof.write(&mut pi_ser) {
        Ok(_) => {
            let s: String = match serde_json::to_string(&pi_ser) {
                Ok(s) => s,
                Err(err) => format!("serde fail, err: {}", err.to_string()),
            };

            return s;
        }
        Err(err) => {
            return format!("pi generate failed, {}", err.to_string());
        }
    };
}

pub fn verify_proof_jni(pi_ser: Vec<u8>) -> Result<bool, ProofError> {
    let test_context = make_test_context_2_to_2();
    let public_inputs: Vec<Scalar> = vec![
        test_context.merkle_rt_1,
        test_context.merkle_rt_2,
        test_context.sn_1,
        test_context.sn_2,
        test_context.cm_1,
        test_context.cm_2,
    ];

    let pi_des: Proof<Bls12> = Proof::read(&*pi_ser.clone()).unwrap();
    let ret = CoinProof::verify_proof_2_to_2(pi_des, &public_inputs, &test_context.hasher);

    ret
}

pub fn pi_gen_1() -> Vec<u8> {
    let test_context = make_test_context_2_to_2();

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

    let coin_2_old = OldCoin {
        addr_pk: Some(test_context.addr_pk_2_old),
        addr_sk: Some(test_context.addr_sk_2_old),
        rho: Some(test_context.rho_2_old),
        r: Some(test_context.r_2_old),
        s: Some(test_context.s_2_old),
        v: Some(test_context.v_2_old),
        cm: Some(test_context.cm_2_old),
        auth_path: test_context.auth_path_2.map(|e| Some(e)),
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

    let proof = CoinProof::generate_proof_2_to_2(coin_1_old, coin_2_old, coin_1_new, coin_2_new)
        .expect("proof should be created");

    let mut pi_ser = Vec::new();

    proof.write(&mut pi_ser).expect("pi should be serialized");

    pi_ser
}

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
    pub merkle_rt_1: Scalar,
    pub sn_1: Scalar,

    // old coin 2
    pub addr_pk_2_old: Scalar,
    pub addr_sk_2_old: Scalar,
    pub r_2_old: Scalar,
    pub s_2_old: Scalar,
    pub rho_2_old: Scalar,
    pub v_2_old: Scalar,
    pub cm_2_old: Scalar,
    pub auth_path_2: [(Scalar, bool); CM_TREE_DEPTH as usize],
    pub merkle_rt_2: Scalar,
    pub sn_2: Scalar,

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

pub fn make_test_context_2_to_2_depth_32() -> TestContext {
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

    let (addr_pk_2_old, addr_sk_2_old, r_2_old, s_2_old, rho_2_old, v_2_old, cm_2_old, sn_2) = {
        let dummy_old_coin = OldCoin::new_dummy().unwrap();

        let sn = hasher.mimc_scalar(dummy_old_coin.addr_sk.unwrap(), dummy_old_coin.rho.unwrap());

        (
            dummy_old_coin.addr_pk.unwrap(),
            dummy_old_coin.addr_sk.unwrap(),
            dummy_old_coin.r.unwrap(),
            dummy_old_coin.s.unwrap(),
            dummy_old_coin.rho.unwrap(),
            dummy_old_coin.v.unwrap(),
            dummy_old_coin.cm.unwrap(),
            sn,
        )
    };

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
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
            let arr = U8Array::from_int(80);
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
            let arr = U8Array::from_int(31);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(32);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(33);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(34);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(20);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let tree_simulator =
        MerkleTreeSim::init(CM_TREE_DEPTH as u32, vec![cm_1_old, cm_2_old]).unwrap();

    let merkle_tree = tree_simulator.merkle_tree;

    let merkle_nodes = tree_simulator.nodes;

    println!("{:#?}", merkle_nodes);

    let merkle_rt_1 = *merkle_nodes
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

    let merkle_rt_2 = *merkle_nodes
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    let auth_path_2 = {
        let v = merkle_tree.generate_auth_paths(1);

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

    println!("auth_path_1: {:#?}", auth_path_1);
    println!("auth_path_2: {:#?}", auth_path_2);

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
        merkle_rt_1,
        sn_1,
        addr_pk_2_old,
        addr_sk_2_old,
        r_2_old,
        s_2_old,
        rho_2_old,
        v_2_old,
        cm_2_old,
        auth_path_2,
        merkle_rt_2,
        sn_2,
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
pub fn make_test_context_2_to_2() -> TestContext {
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

    let (addr_pk_2_old, addr_sk_2_old, r_2_old, s_2_old, rho_2_old, v_2_old, cm_2_old, sn_2) = {
        let dummy_old_coin = OldCoin::new_dummy().unwrap();

        let sn = hasher.mimc_scalar(dummy_old_coin.addr_sk.unwrap(), dummy_old_coin.rho.unwrap());

        (
            dummy_old_coin.addr_pk.unwrap(),
            dummy_old_coin.addr_sk.unwrap(),
            dummy_old_coin.r.unwrap(),
            dummy_old_coin.s.unwrap(),
            dummy_old_coin.rho.unwrap(),
            dummy_old_coin.v.unwrap(),
            dummy_old_coin.cm.unwrap(),
            sn,
        )
    };

    let (addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
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
            let arr = U8Array::from_int(80);
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
            let arr = U8Array::from_int(31);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = {
            let arr = U8Array::from_int(32);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let s = {
            let arr = U8Array::from_int(33);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let rho = {
            let arr = U8Array::from_int(34);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let v = {
            let arr = U8Array::from_int(10);
            ScalarExt::parse_arr(&arr).unwrap()
        };

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let tree_simulator =
        MerkleTreeSim::init(CM_TREE_DEPTH as u32, vec![cm_1_old, cm_2_old]).unwrap();

    let merkle_tree = tree_simulator.merkle_tree;

    let merkle_nodes = tree_simulator.nodes;

    println!("{:#?}", merkle_nodes);

    let merkle_rt_1 = *merkle_nodes
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

    let merkle_rt_2 = *merkle_nodes
        .get(format!("{}_0", CM_TREE_DEPTH).as_str())
        .unwrap();

    let auth_path_2 = {
        let v = merkle_tree.generate_auth_paths(1);

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

    println!("auth_path_1: {:#?}", auth_path_1);
    println!("auth_path_2: {:#?}", auth_path_2);

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
        merkle_rt_1,
        sn_1,
        addr_pk_2_old,
        addr_sk_2_old,
        r_2_old,
        s_2_old,
        rho_2_old,
        v_2_old,
        cm_2_old,
        auth_path_2,
        merkle_rt_2,
        sn_2,
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
