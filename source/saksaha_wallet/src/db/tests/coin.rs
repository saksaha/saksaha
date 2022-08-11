use crate::{
    db::{tests::make_dummy_db, WalletDB, USER_1, USER_2},
    types::Status,
};

use sak_crypto::{Hasher, Scalar, ScalarExt};
use sak_types::U8Array;

async fn get_dummy_random_gen_coin() -> (
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Status,
) {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(sak_crypto::rand() as u64).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(sak_crypto::rand() as u64);
    let r = U8Array::from_int(sak_crypto::rand() as u64);
    let s = U8Array::from_int(sak_crypto::rand() as u64);
    let v = U8Array::from_int((sak_crypto::rand() / 10000) as u64);
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

    let status = Status::Unused;

    (
        addr_pk,
        ScalarExt::parse_arr(&addr_sk).unwrap(),
        ScalarExt::parse_arr(&rho).unwrap(),
        ScalarExt::parse_arr(&r).unwrap(),
        ScalarExt::parse_arr(&s).unwrap(),
        ScalarExt::parse_arr(&v).unwrap(),
        cm,
        status,
    )
}

async fn get_dummy_coin(
    rho: u64,
    r: u64,
    s: u64,
    v: u64,
    addr_sk: u64,
) -> (
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    Status,
) {
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

    let status = Status::Unused;

    (
        addr_pk,
        ScalarExt::parse_arr(&addr_sk).unwrap(),
        ScalarExt::parse_arr(&rho).unwrap(),
        ScalarExt::parse_arr(&r).unwrap(),
        ScalarExt::parse_arr(&s).unwrap(),
        ScalarExt::parse_arr(&v).unwrap(),
        cm,
        status,
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_db_store_randomly_generated_coins() {
    sak_test_utils::init_test_log();

    let db = make_dummy_db().await;

    for idx in 0..5 {
        let (addr_pk, addr_sk, rho, r, s, v, cm, status) =
            get_dummy_random_gen_coin().await;

        db.schema
            .put_coin(
                &cm.to_string(),
                &rho.to_string(),
                &r.to_string(),
                &s.to_string(),
                &v.to_string(),
                &addr_pk.to_string(),
                &addr_sk.to_string(),
                &USER_1.to_string(),
                &status,
                &idx,
            )
            .await
            .unwrap();
    }

    for idx in 5..10 {
        let (addr_pk, addr_sk, rho, r, s, v, cm, status) =
            get_dummy_random_gen_coin().await;

        db.schema
            .put_coin(
                &cm.to_string(),
                &rho.to_string(),
                &r.to_string(),
                &s.to_string(),
                &v.to_string(),
                &addr_pk.to_string(),
                &addr_sk.to_string(),
                &USER_2.to_string(),
                &status,
                &idx,
            )
            .await
            .unwrap();
    }

    let latest_cm_idx = db.schema.get_latest_cm_idx().unwrap().unwrap();
    println!("latest_cm_idx: {:?}", latest_cm_idx);

    for idx in 0..5 {
        let cm = db.schema.get_cm(&idx).await.unwrap().unwrap();
        let user_id = db.schema.get_user_id(&cm).await.unwrap().unwrap();

        println!("[+] user_id: {:?}, USER_1: {:?}", user_id, USER_1);
        assert_eq!(user_id, USER_1);
    }

    for idx in 5..10 {
        let cm = db.schema.get_cm(&idx).await.unwrap().unwrap();
        let user_id = db.schema.get_user_id(&cm).await.unwrap().unwrap();

        println!("[+] user_id: {:?}, USER_2: {:?}", user_id, USER_2);
        assert_eq!(user_id, USER_2);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_db_store_coins() {
    sak_test_utils::init_test_log();

    let db = make_dummy_db().await;

    for idx in 0..5 {
        let (addr_pk, addr_sk, rho, r, s, v, cm, status) = get_dummy_coin(
            10 + idx,
            10 + idx,
            10 + idx,
            100 + idx * 100,
            10 + idx,
        )
        .await;

        db.schema
            .put_coin(
                &cm.to_string(),
                &rho.to_string(),
                &r.to_string(),
                &s.to_string(),
                &v.to_string(),
                &addr_pk.to_string(),
                &addr_sk.to_string(),
                &USER_1.to_string(),
                &status,
                &(idx as u128),
            )
            .await
            .unwrap();
    }

    for idx in 0..5 {
        let cm = db.schema.get_cm(&idx).await.unwrap().unwrap();
        let rho = db.schema.get_rho(&cm).await.unwrap().unwrap();
        let r = db.schema.get_r(&cm).await.unwrap().unwrap();
        let s = db.schema.get_s(&cm).await.unwrap().unwrap();
        let v = db.schema.get_v(&cm).await.unwrap().unwrap();
        let addr_sk = db.schema.get_a_sk(&cm).await.unwrap().unwrap();
        let user_id = db.schema.get_user_id(&cm).await.unwrap().unwrap();
        let status = db.schema.get_status(&cm).await.unwrap().unwrap();

        assert_eq!(user_id, USER_1);

        assert_eq!(status, Status::Unused);

        assert_eq!(
            rho,
            ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
                .unwrap()
                .to_string()
        );
        assert_eq!(
            r,
            ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
                .unwrap()
                .to_string()
        );
        assert_eq!(
            s,
            ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
                .unwrap()
                .to_string()
        );
        assert_eq!(
            addr_sk,
            ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) + 10))
                .unwrap()
                .to_string()
        );
        assert_eq!(
            v,
            ScalarExt::parse_arr(&U8Array::from_int(&(idx as u64) * 100 + 100))
                .unwrap()
                .to_string()
        );
    }
}
