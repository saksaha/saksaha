use sak_crypto::{Hasher, Scalar, ScalarExt};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::U8Array;
use saksaha::{generate_proof_1_to_2, get_auth_path, send_tx_mint};
use std::{collections::HashMap, time::Duration};

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

fn generate_a_coin(value: u64) -> Coin {
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

    // 1. User mint a new coin (coin_1_old)
    let coin_1_old = generate_a_coin(100);

    // 2. send tx_mint to the blockchain
    {
        let ctr_addr: Option<String> = None;
        let req_type = String::from("");
        let arg: HashMap<String, String> = HashMap::new();

        let resp = send_tx_mint(
            ctr_addr,
            req_type,
            arg,
            coin_1_old.cm,
            coin_1_old.v,
            coin_1_old.k,
            coin_1_old.s,
        )
        .await
        .unwrap();

        println!("[+] Response: {:#?}", resp);

        println!("[+] Sleep 6 seconds...");
        tokio::time::sleep(Duration::from_secs(6)).await;
    }

    // 3. generate 2 new coins (coin_1_new, coin_2_new)
    let coin_1_new = generate_a_coin(60);

    let coin_2_new = generate_a_coin(40);

    // 4. generate a proof for tx_pour
    let cm_1_old_idx: u128 = 2;

    let auth_path_json_resp = get_auth_path(cm_1_old_idx).await.unwrap();

    let auth_path = auth_path_json_resp.result.unwrap().result;

    let auth_path: Vec<Option<(Scalar, bool)>> = auth_path
        .into_iter()
        .map(|e| match e {
            Some(n) => {
                let node_value = n.0;
                Some((ScalarExt::parse_arr(&node_value).unwrap(), n.1))
            }
            None => {
                panic!()
            }
        })
        .collect();

    let auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize] =
        auth_path.try_into().unwrap();

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

    println!("[+] Waiting for proof...");

    let pi = generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new).await;

    println!("[!] pi: {:#?}", pi);

    // `saksaha_network` should valid the `proof`
}
