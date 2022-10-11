use crate::v0::testing::values;
use crate::TxCandidate;
use crate::{
    mock_coin_custom, Cm, MerkleRt, MintTxCandidate, PourTxCandidate, Sn, Tx, VALIDATOR,
    VALIDATOR_CTR_ADDR,
};
use sak_crypto::hasher::MiMC;
use sak_crypto::{rand, Scalar};
use sak_crypto::{MerkleTreeSim, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_proof::NewCoin;
use sak_proof::OldCoin;
use sak_proof::{CoinProof, DUMMY_MERKLE_RT, DUMMY_SN};
use type_extension::U8Array;

pub fn mock_pour_tc_custom(
    pi: Vec<u8>,
    sns: Vec<Sn>,
    cms: Vec<Cm>,
    merkle_rts: Vec<MerkleRt>,
) -> TxCandidate {
    let tc = PourTxCandidate::new(
        String::from("created_at_test"),
        vec![44, 44, 44],
        String::from("author_sig_test"),
        Some(String::from("ctr_addr_test")),
        pi,
        sns,
        cms,
        merkle_rts,
    );

    TxCandidate::Pour(tc)
}

pub fn mock_pour_tx_custom(
    pi: Vec<u8>,
    sns: Vec<Sn>,
    cms: Vec<Cm>,
    merkle_rts: Vec<MerkleRt>,
) -> Tx {
    let c = mock_pour_tc_custom(pi, sns, cms, merkle_rts);

    c.upgrade(0)
}

// TODO This should change
pub fn mock_pour_tc_random() -> TxCandidate {
    let hasher = MiMC::new();

    let dummy_coin = mock_coin_custom(0, 0, 0, 0, 0);
    let dummy_auth_path = [
        Some((Scalar::default(), false));CM_TREE_DEPTH as usize
        // Some((Scalar::default(), false)),
        // Some((Scalar::default(), false)),
        // Some((Scalar::default(), false)),
        // Some((Scalar::default(), false)),
        // Some((Scalar::default(), false)),
        // Some((Scalar::default(), false)),
    ];

    let (addr_pk_1_old, addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old, cm_1_old, sn_1) = {
        let addr_sk = ScalarExt::parse_u64(rand() as u64).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(rand() as u64).unwrap();

        let s = ScalarExt::parse_u64(rand() as u64).unwrap();

        let rho = ScalarExt::parse_u64(rand() as u64).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(1000)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

    let (_addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
        let addr_sk = ScalarExt::parse_u64(rand() as u64).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(rand() as u64).unwrap();

        let s = ScalarExt::parse_u64(rand() as u64).unwrap();

        let rho = ScalarExt::parse_u64(rand() as u64).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(590)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (_addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk = ScalarExt::parse_u64(rand() as u64).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(rand() as u64).unwrap();

        let s = ScalarExt::parse_u64(rand() as u64).unwrap();

        let rho = ScalarExt::parse_u64(rand() as u64).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(400)).unwrap();

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
        let mut ret = [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            ret[idx] = Some((merkle_node, p.direction));
        });

        ret
    };

    let coin_1_old = OldCoin {
        addr_pk: Some(addr_pk_1_old),
        addr_sk: Some(addr_sk_1_old),
        rho: Some(rho_1_old),
        r: Some(r_1_old),
        s: Some(s_1_old),
        v: Some(v_1_old),
        cm: Some(cm_1_old),
        auth_path: auth_path_1,
    };

    let dummy_coin = OldCoin {
        addr_pk: Some(ScalarExt::parse_arr(&dummy_coin.addr_pk).unwrap()),
        addr_sk: Some(ScalarExt::parse_arr(&dummy_coin.addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&dummy_coin.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&dummy_coin.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&dummy_coin.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&dummy_coin.v).unwrap()),
        cm: Some(ScalarExt::parse_arr(&dummy_coin.cm).unwrap()),
        auth_path: dummy_auth_path,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(addr_pk_1),
        rho: Some(rho_1),
        r: Some(r_1),
        s: Some(s_1),
        v: Some(v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(addr_pk_2),
        rho: Some(rho_2),
        r: Some(r_2),
        s: Some(s_2),
        v: Some(v_2),
    };

    let pi =
        CoinProof::generate_proof_2_to_2(coin_1_old, dummy_coin, coin_1_new, coin_2_new).unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi).unwrap();

    let pour_tc = PourTxCandidate::new(
        "created_at".to_string(),
        vec![],
        "author_sig".to_string(),
        None,
        pi_serialized,
        vec![sn_1.to_bytes(), DUMMY_SN],
        vec![cm_1.to_bytes(), cm_2.to_bytes()],
        vec![merkle_rt.to_bytes(), DUMMY_MERKLE_RT],
    );

    let c = TxCandidate::Pour(pour_tc);

    c
}

pub fn mock_pour_tc_1() -> TxCandidate {
    let hasher = MiMC::new();

    let (addr_pk_1_old, addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old, cm_1_old, sn_1) = {
        let addr_sk = values::get_addr_sk_1();

        let addr_sk = ScalarExt::parse_arr(&addr_sk).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(0).unwrap();

        let s = ScalarExt::parse_u64(0).unwrap();

        let rho = ScalarExt::parse_u64(0).unwrap();

        let v = ScalarExt::parse_u64(1000).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

    let (_addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
        let addr_sk = values::get_addr_sk_1();

        let addr_sk = ScalarExt::parse_arr(&addr_sk).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(0).unwrap();

        let s = ScalarExt::parse_u64(0).unwrap();

        let rho = ScalarExt::parse_u64(0).unwrap();

        let v = ScalarExt::parse_u64(590).unwrap(); // Subtract GAS!

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (_addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk = values::get_addr_sk_1();

        let addr_sk = ScalarExt::parse_arr(&addr_sk).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_u64(0).unwrap();

        let s = ScalarExt::parse_u64(0).unwrap();

        let rho = ScalarExt::parse_u64(0).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(400)).unwrap();

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

        let mut ret = [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    let coin_1_old = OldCoin {
        addr_pk: Some(addr_pk_1_old),
        addr_sk: Some(addr_sk_1_old),
        rho: Some(rho_1_old),
        r: Some(r_1_old),
        s: Some(s_1_old),
        v: Some(v_1_old),
        cm: Some(cm_1_old),
        auth_path: auth_path_1,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(addr_pk_1),
        rho: Some(rho_1),
        r: Some(r_1),
        s: Some(s_1),
        v: Some(v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(addr_pk_2),
        rho: Some(rho_2),
        r: Some(r_2),
        s: Some(s_2),
        v: Some(v_2),
    };

    let pi = CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new).unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi).unwrap();

    let pour_tc = PourTxCandidate::new(
        "created_at".to_string(),
        vec![],
        "author_sig".to_string(),
        None,
        pi_serialized,
        vec![sn_1.to_bytes(), DUMMY_SN],
        vec![cm_1.to_bytes(), cm_2.to_bytes()],
        vec![merkle_rt.to_bytes(), DUMMY_MERKLE_RT],
    );

    let c = TxCandidate::Pour(pour_tc);

    c
}

pub fn mock_pour_tc_invalid_pi() -> TxCandidate {
    let hasher = MiMC::new();

    let (addr_pk_1_old, addr_sk_1_old, r_1_old, s_1_old, rho_1_old, v_1_old, cm_1_old, sn_1) = {
        let addr_sk = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let rho = ScalarExt::parse_arr(&U8Array::from_int(0)).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(1000)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        let sn = hasher.mimc_scalar(addr_sk, rho);

        (addr_pk, addr_sk, r, s, rho, v, cm, sn)
    };

    let (_addr_sk_1, addr_pk_1, r_1, s_1, rho_1, v_1, cm_1) = {
        let addr_sk = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let rho = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(600)).unwrap();

        let cm = {
            let k = hasher.comm2_scalar(r, addr_pk, rho);

            hasher.comm2_scalar(s, v, k)
        };

        (addr_sk, addr_pk, r, s, rho, v, cm)
    };

    let (_addr_sk_2, addr_pk_2, r_2, s_2, rho_2, v_2, cm_2) = {
        let addr_sk = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

        let r = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let s = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let rho = ScalarExt::parse_arr(&U8Array::from_int(rand() as u64 / 100)).unwrap();

        let v = ScalarExt::parse_arr(&U8Array::from_int(400)).unwrap();

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
        let mut ret = [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

        v.iter().enumerate().for_each(|(idx, p)| {
            if idx >= ret.len() {
                panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
            }

            let key = format!("{}_{}", idx, p.idx);
            let merkle_node = match merkle_nodes.get(key.as_str()) {
                Some(t) => *t,
                None => Scalar::default(),
            };

            ret[idx] = Some((merkle_node.clone(), p.direction));
        });

        ret
    };

    let coin_1_old = OldCoin {
        addr_pk: Some(addr_pk_1_old),
        addr_sk: Some(addr_sk_1_old),
        rho: Some(rho_1_old),
        r: Some(r_1_old),
        s: Some(s_1_old),
        v: Some(v_1_old),
        cm: Some(cm_1_old),
        auth_path: auth_path_1,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(addr_pk_1),
        rho: Some(rho_1),
        r: Some(r_1),
        s: Some(s_1),
        v: Some(v_1),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(addr_pk_2),
        rho: Some(rho_2),
        r: Some(r_2),
        s: Some(s_2),
        v: Some(v_2),
    };

    let pi = CoinProof::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new).unwrap();

    let pi_serialized = CoinProof::serialize_pi(&pi).unwrap();

    let pour_tc = PourTxCandidate::new(
        "created_at".to_string(),
        vec![],
        "author_sig".to_string(),
        None,
        pi_serialized,
        vec![sn_1.to_bytes()],
        vec![cm_1.to_bytes(), cm_2.to_bytes()],
        vec![merkle_rt.to_bytes()],
    );

    let c = TxCandidate::Pour(pour_tc);

    c
}

pub fn mock_mint_tc(cm: [u8; 32], v: [u8; 32], k: [u8; 32], s: [u8; 32]) -> TxCandidate {
    let validator_wasm = VALIDATOR.to_vec();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_custom_1"),
        validator_wasm,
        String::from("author_sig_mint_custom_1"),
        Some(VALIDATOR_CTR_ADDR.to_string()),
        vec![cm],
        v,
        k,
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_random() -> TxCandidate {
    let hasher = MiMC::new();

    let v = ScalarExt::parse_arr(&U8Array::from_int(400)).unwrap();

    let r = ScalarExt::parse_u64(rand() as u64).unwrap();

    let s = ScalarExt::parse_u64(rand() as u64).unwrap();

    let rho = ScalarExt::parse_u64(rand() as u64).unwrap();

    let addr_sk = ScalarExt::parse_u64(rand() as u64).unwrap();

    let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();

    let k = hasher.comm2_scalar(r, addr_pk, rho);

    let cm = hasher.comm2_scalar(s, v, k);

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_1"),
        vec![],
        String::from("author_sig_mint_1"),
        None,
        vec![cm.to_bytes()],
        v.to_bytes(),
        k.to_bytes(),
        s.to_bytes(),
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_1() -> TxCandidate {
    let validator_wasm = VALIDATOR.to_vec();

    let hasher = MiMC::new();

    let v = U8Array::from_int(1000);

    println!("v: {:?}", v);

    let rho = U8Array::from_int(1);

    let r = U8Array::from_int(2);

    let s = U8Array::from_int(3);

    let addr_sk = U8Array::from_int(4);

    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();

    let k = hasher.comm2(&r, &addr_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_1"),
        validator_wasm,
        String::from("author_sig_mint_1"),
        Some(VALIDATOR_CTR_ADDR.to_string()),
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_2() -> TxCandidate {
    let hasher = MiMC::new();

    let v = U8Array::from_int(1000);

    let rho = U8Array::new_empty_32();

    let r = U8Array::new_empty_32();

    let s = U8Array::new_empty_32();

    let addr_sk = U8Array::new_empty_32();

    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();

    let k = hasher.comm2(&r, &addr_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    println!("mock mint tc 2 cm: {:?}", cm.to_bytes());

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_2"),
        vec![2],
        String::from("author_sig_mint_2"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_3() -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::from_int(0x11);

    let r = U8Array::from_int(0x12);

    let s = U8Array::from_int(0x13);

    let v = U8Array::from_int(100);

    let a_sk = U8Array::from_int(0x14);

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_3"),
        vec![3],
        String::from("author_sig_mint_3"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_4() -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::from_int(0x21);

    let r = U8Array::from_int(0x22);

    let s = U8Array::from_int(0x23);

    let v = U8Array::from_int(100);

    let a_sk = U8Array::from_int(0x24);

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_4"),
        vec![4],
        String::from("author_sig_mint_4"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_5() -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::from_int(0x31);

    let r = U8Array::from_int(0x32);

    let s = U8Array::from_int(0x33);

    let v = U8Array::from_int(100);

    let a_sk = U8Array::from_int(0x34);

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_5"),
        vec![5],
        String::from("author_sig_mint_5"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_6() -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::from_int(0x41);

    let r = U8Array::from_int(0x42);

    let s = U8Array::from_int(0x43);

    let v = U8Array::from_int(100);

    let a_sk = U8Array::from_int(0x44);

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_6"),
        vec![6],
        String::from("author_sig_mint_6"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_dummy_old_coin() -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::from_int(0);

    let r = U8Array::from_int(0);

    let s = U8Array::from_int(0);

    let v = U8Array::from_int(0);

    let a_sk = U8Array::from_int(0);

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    // CM : 0x3bb4c03f8e718ec58f4f2bb2b2fb83149b5fe59a75c5c98893e40c56bb3e8deb

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_5"),
        vec![5],
        String::from("author_sig_mint_5"),
        None,
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}

pub fn mock_mint_tc_deploying_contract(contract_data: Vec<u8>, ctrt_addr: String) -> TxCandidate {
    let hasher = MiMC::new();

    let rho = U8Array::new_empty_32();

    let r = U8Array::new_empty_32();

    let s = U8Array::new_empty_32();

    let v = U8Array::new_empty_32();

    let a_sk = U8Array::new_empty_32();

    let a_pk = hasher
        .mimc_single_scalar(ScalarExt::parse_arr(&a_sk).unwrap())
        .unwrap();

    let k = hasher.comm2(&r, &a_pk.to_bytes(), &rho).unwrap();

    let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

    let tx_candidate = MintTxCandidate::new(
        String::from("created_at_mint_3"),
        contract_data,
        String::from("author_sig_mint_3"),
        Some(ctrt_addr),
        vec![cm.to_bytes()],
        v,
        k.to_bytes(),
        s,
    );

    TxCandidate::Mint(tx_candidate)
}
