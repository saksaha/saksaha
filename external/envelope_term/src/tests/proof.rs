use async_trait::async_trait;
use sak_crypto::{Bls12, Hasher, Proof, Scalar, ScalarExt};
use sak_dist_ledger::{
    Consensus, ConsensusError, DistLedger, DistLedgerApis, DistLedgerArgs,
};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::{BlockCandidate, TxCandidate, U8Array};
use saksaha::{
    generate_proof_1_to_2, get_auth_path, send_tx_mint, verify_proof_1_to_2,
};
use std::{collections::HashMap, sync::Arc, time::Duration};

pub struct DummyPos {}

#[async_trait]
impl Consensus for DummyPos {
    async fn do_consensus(
        &self,
        _dist_ledger_apis: &DistLedgerApis,
        _txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}

pub(crate) fn make_dummy_genesis_block(tx: TxCandidate) -> BlockCandidate {
    let genesis_block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![tx],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    genesis_block
}

pub(crate) fn make_dummy_pos() -> Box<DummyPos> {
    Box::new(DummyPos {})
}

pub(crate) async fn make_dist_ledger(block: BlockCandidate) -> DistLedger {
    let pos = make_dummy_pos();

    let dist_ledger_args = DistLedgerArgs {
        app_prefix: String::from("test"),
        tx_sync_interval: None,
        genesis_block: Some(block),
        consensus: pos,
        block_sync_interval: None,
    };

    let dist_ledger = DistLedger::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub struct Coin {
    pub addr_sk: [u8; 32],
    pub addr_pk: [u8; 32],
    pub rho: [u8; 32],
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: [u8; 32],
    pub k: [u8; 32],
    pub cm: [u8; 32],
}

fn generate_a_dummy_coin(value: u64) -> Coin {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(sak_crypto::rand() as u64).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(sak_crypto::rand() as u64);
    let r = U8Array::from_int(sak_crypto::rand() as u64);
    let s = U8Array::from_int(sak_crypto::rand() as u64);
    let v = U8Array::from_int(value);

    let k = hasher.comm2_scalar(
        ScalarExt::parse_arr(&r).unwrap(),
        addr_pk,
        ScalarExt::parse_arr(&rho).unwrap(),
    );
    let cm = hasher.comm2_scalar(
        ScalarExt::parse_arr(&s).unwrap(),
        ScalarExt::parse_arr(&v).unwrap(),
        k,
    );

    println!("\n[*] New Coin Info!");
    println!("[-] addr_sk: {:?}", ScalarExt::parse_arr(&addr_sk));
    println!("[-] addr_pk: {:?}", addr_pk);
    println!("[-] rho: {:?}", ScalarExt::parse_arr(&rho));
    println!("[-] r: {:?}", ScalarExt::parse_arr(&r));
    println!("[-] s: {:?}", ScalarExt::parse_arr(&s));
    println!("[-] v: {:?}", ScalarExt::parse_arr(&v));
    println!("[-] k: {:?}", k);
    println!("[-] cm: {:?}", cm);

    Coin {
        addr_sk,
        addr_pk: addr_pk.to_bytes(),
        rho,
        r,
        s,
        v,
        k: k.to_bytes(),
        cm: cm.to_bytes(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_generate_a_proof() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let coin_1_old = generate_a_dummy_coin(100);

    let tx = TxCandidate::new_dummy_mint_custom(
        coin_1_old.cm,
        coin_1_old.v,
        coin_1_old.k,
        coin_1_old.s,
    );
    let genesis_block = make_dummy_genesis_block(tx);
    let dist_ledger = make_dist_ledger(genesis_block).await;

    // 4. generate a proof for tx_pour
    let cm_1_old_idx: u128 = 0;

    let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);
    let auth_path_idx = merkle_tree.generate_auth_paths(cm_1_old_idx);

    let mut auth_path =
        [Some((Scalar::default(), false)); CM_TREE_DEPTH as usize];

    println!("[*] initial auth_path: {:#?}", auth_path);

    for (idx, p) in auth_path_idx.iter().enumerate() {
        if idx >= auth_path.len() {
            panic!("Invalid assignment to a fixed sized array, idx: {}", idx);
        }

        println!("auth_path: {}_{}", idx, p.idx);
        let key = format!("{}_{}", idx, p.idx);

        let merkle_node = dist_ledger.apis.get_merkle_node(&key).await.unwrap();

        let merkle_node = ScalarExt::parse_arr(&merkle_node).unwrap();

        auth_path[idx] = Some((merkle_node, p.direction));
    }

    println!("[*] updated auth_path: {:#?}", auth_path);

    let coin_1_new = generate_a_dummy_coin(60);
    let coin_2_new = generate_a_dummy_coin(40);

    let coin_1_old = OldCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_1_old.addr_pk).unwrap()),
        addr_sk: Some(ScalarExt::parse_arr(&coin_1_old.addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_1_old.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_1_old.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_1_old.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_1_old.v).unwrap()),
        cm: Some(ScalarExt::parse_arr(&coin_1_old.cm).unwrap()),
        auth_path,
    };

    let coin_1_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_1_new.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_1_new.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_1_new.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_1_new.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_1_new.v).unwrap()),
    };

    let coin_2_new = NewCoin {
        addr_pk: Some(ScalarExt::parse_arr(&coin_2_new.addr_pk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&coin_2_new.rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&coin_2_new.r).unwrap()),
        s: Some(ScalarExt::parse_arr(&coin_2_new.s).unwrap()),
        v: Some(ScalarExt::parse_arr(&coin_2_new.v).unwrap()),
    };

    println!("\n[+] Waiting for generating pi...");
    let pi = generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
        .await
        .unwrap();

    println!("[!] pi: {:#?}", pi);

    {
        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized, len: {}, {:?}", pi_ser.len(), pi_ser);

        let pi_des: Proof<Bls12> = Proof::read(&*pi_ser).unwrap();

        println!("[!] pi deserialized: {:#?}", pi_des);
    }

    println!("\n[+] Verifying  pi...");

    {
        let hasher = Hasher::new();

        let merkle_rt = {
            let mut merkle_rt = coin_1_old.cm.unwrap();

            for i in auth_path {
                let (v, _) = i.unwrap();

                merkle_rt = hasher.mimc_scalar(merkle_rt, v);
            }

            merkle_rt
        };

        let sn_1_old = hasher.mimc_scalar(
            coin_1_old.clone().addr_sk.unwrap(),
            coin_1_old.clone().rho.unwrap(),
        );

        let cm_1_new = {
            let k = hasher.comm2_scalar(
                coin_1_new.clone().r.unwrap(),
                coin_1_new.clone().addr_pk.unwrap(),
                coin_1_new.clone().rho.unwrap(),
            );

            let cm_1_new = hasher.comm2_scalar(
                coin_1_new.clone().s.unwrap(),
                coin_1_new.clone().v.unwrap(),
                k,
            );

            cm_1_new
        };

        let cm_2_new = {
            let k = hasher.comm2_scalar(
                coin_2_new.clone().r.unwrap(),
                coin_2_new.clone().addr_pk.unwrap(),
                coin_2_new.clone().rho.unwrap(),
            );

            let cm_2_new = hasher.comm2_scalar(
                coin_2_new.clone().s.unwrap(),
                coin_2_new.clone().v.unwrap(),
                k,
            );

            cm_2_new
        };

        let public_inputs = [merkle_rt, sn_1_old, cm_1_new, cm_2_new];

        assert_eq!(
            verify_proof_1_to_2(pi, &public_inputs, &hasher).await,
            true
        );
    }
}
