use crate::{
    db::{tests::make_dummy_db, WalletDB, USER_1, USER_2},
    WalletError,
};
use sak_crypto::{Hasher, Scalar, ScalarExt};
use sak_proofs::OldCoin;
use sak_types::{CoinRecord, CoinStatus};
use type_extension::U8Array;

struct TestWalletCoin {
    addr_pk: Scalar,
    addr_sk: Scalar,
    rho: Scalar,
    r: Scalar,
    s: Scalar,
    v: Scalar,
    cm: Scalar,
    status: CoinStatus,
}

fn mock_coin_record_1() -> Result<CoinRecord, WalletError> {
    let hasher = Hasher::new();

    let (addr_sk, addr_pk) = {
        let addr_sk = U8Array::from_int(20);
        let addr_pk = hasher.mimc_single(&addr_sk)?;

        let addr_sk_scalar = ScalarExt::parse_arr(&addr_sk)?;

        (addr_sk_scalar, addr_pk)
    };

    let rho = {
        let arr = U8Array::from_int(21);
        ScalarExt::parse_arr(&arr)?
    };

    let r = {
        let arr = U8Array::from_int(22);
        ScalarExt::parse_arr(&arr)?
    };

    let s = {
        let arr = U8Array::from_int(23);
        ScalarExt::parse_arr(&arr)?
    };

    let v = {
        let arr = U8Array::from_int(24);
        ScalarExt::parse_arr(&arr)?
    };

    let k = hasher.comm2_scalar(r, addr_pk, rho);

    let cm = hasher.comm2_scalar(s, v, k);

    let coin_status = CoinStatus::Unused;

    let cm_idx = 0;

    let coin = CoinRecord {
        addr_pk,
        addr_sk,
        rho,
        r,
        s,
        v,
        cm,
        coin_status,
        cm_idx,
        coin_idx: None,
    };

    Ok(coin)
}

async fn make_coin_record(
    rho: u64,
    r: u64,
    s: u64,
    v: u64,
    addr_sk: u64,
) -> TestWalletCoin {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(addr_sk).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(rho);
    let r = U8Array::from_int(r);
    let s = U8Array::from_int(s);
    let v = U8Array::from_int(v);
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

    let status = CoinStatus::Unused;

    TestWalletCoin {
        addr_pk,
        addr_sk: ScalarExt::parse_arr(&addr_sk).unwrap(),
        rho: ScalarExt::parse_arr(&rho).unwrap(),
        r: ScalarExt::parse_arr(&r).unwrap(),
        s: ScalarExt::parse_arr(&s).unwrap(),
        v: ScalarExt::parse_arr(&v).unwrap(),
        cm,
        status,
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_db_store_coin_record() {
    sak_test_utils::init_test_log();

    let db = make_dummy_db().await;

    let coin = mock_coin_record_1().unwrap();

    // db.schema
    //     .put_coin(
    //         coin,
    //         // &cm.to_string(),
    //         // &rho.to_string(),
    //         // &r.to_string(),
    //         // &s.to_string(),
    //         // &v.to_string(),
    //         // &addr_pk.to_string(),
    //         // &addr_sk.to_string(),
    //         // &USER_1.to_string(),
    //         // &status,
    //         // &idx,
    //     )
    //     .await
    //     .unwrap();

    // let latest_cm_idx = db.schema.get_latest_cm_idx().unwrap().unwrap();
    // println!("latest_cm_idx: {:?}", latest_cm_idx);

    // let cm = db.schema.get_cm(&idx).unwrap().unwrap();
    // let user_id = db.schema.get_user_id(&cm).unwrap().unwrap();

    // println!("[+] user_id: {:?}, USER_1: {:?}", user_id, USER_1);
    // assert_eq!(user_id, USER_1);
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_wallet_db_store_coins() {
//     sak_test_utils::init_test_log();

//     let db = make_dummy_db().await;

//     for idx in 0..5 {
//         let (addr_pk, addr_sk, rho, r, s, v, cm, status) = get_dummy_coin(
//             10 + idx,
//             10 + idx,
//             10 + idx,
//             100 + idx * 100,
//             10 + idx,
//         )
//         .await;

//         db.schema
//             .put_coin(
//                 &cm.to_string(),
//                 &rho.to_string(),
//                 &r.to_string(),
//                 &s.to_string(),
//                 &v.to_string(),
//                 &addr_pk.to_string(),
//                 &addr_sk.to_string(),
//                 &USER_1.to_string(),
//                 &status,
//             )
//             .await
//             .unwrap();
//     }

//     for idx in 0..5 {
//         let cm = db.schema.get_cm(&idx).unwrap().unwrap();
//         let rho = db.schema.get_rho(&cm).unwrap().unwrap();
//         let r = db.schema.get_r(&cm).unwrap().unwrap();
//         let s = db.schema.get_s(&cm).unwrap().unwrap();
//         let v = db.schema.get_v(&cm).unwrap().unwrap();
//         let addr_sk = db.schema.get_a_sk(&cm).unwrap().unwrap();
//         let user_id = db.schema.get_user_id(&cm).unwrap().unwrap();
//         let status = db.schema.get_status(&cm).await.unwrap().unwrap();

//         assert_eq!(user_id, USER_1);

//         assert_eq!(status, Status::Unused);

//         assert_eq!(
//             rho,
//             ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
//                 .unwrap()
//                 .to_string()
//         );

//         assert_eq!(
//             r,
//             ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
//                 .unwrap()
//                 .to_string()
//         );

//         assert_eq!(
//             s,
//             ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
//                 .unwrap()
//                 .to_string()
//         );

//         assert_eq!(
//             addr_sk,
//             ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
//                 .unwrap()
//                 .to_string()
//         );

//         assert_eq!(
//             v,
//             ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) * 100 + 100))
//                 .unwrap()
//                 .to_string()
//         );
//     }
// }
