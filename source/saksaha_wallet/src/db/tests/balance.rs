use crate::{
    db::{tests::make_dummy_db, WalletDB, USER_1},
    WalletError,
};

use sak_crypto::{rand, Hasher, Scalar, ScalarExt};
use sak_types::U8Array;

async fn get_dummy_random_gen_coins(
) -> (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar, Scalar, bool) {
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

    let status = true;

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
async fn test_wallet_get_all_coins() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let db = make_dummy_db().await;

    for idx in 0..9 {
        let (addr_pk, addr_sk, rho, r, s, v, cm, status) =
            get_dummy_random_gen_coins().await;

        db.schema
            .put_coin_data(
                &cm.to_string(),
                &rho.to_string(),
                &r.to_string(),
                &s.to_string(),
                &v.to_string(),
                &addr_pk.to_string(),
                &addr_sk.to_string(),
                &USER_1.to_string(),
                &status.to_string(),
                &idx,
            )
            .await
            .unwrap();
    }

    let latest_cm_idx = db.schema.get_latest_cm_idx().unwrap().unwrap();
    println!("latest_cm_idx: {:?}", latest_cm_idx);
}
