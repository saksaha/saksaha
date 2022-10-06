use crate::{credential::WalletCredential, db::WalletDB};
use sak_crypto::{hasher::MiMC, ScalarExt};
use sak_types::{CoinRecord, CoinStatus};
use type_extension::U8Array;

pub(crate) fn mock_wallet_credential() -> WalletCredential {
    let public_key = String::from(
        "046f737a049d48f626534328a6aa9507ae6a989750\
        59a9892cab16b8ce748dd265e3abfd928b8bc040683\
        0c10a8e8d2d1159b8c0eed26e2a2d522ad0d44efb6649",
    );

    let secret = String::from(
        "c151588543dc41774d85fee0d4d73dee3c8071d\
        8ebef9b3850be3f66cf0d83b9",
    );

    let c = WalletCredential::load(&public_key, &secret).unwrap();
    c
}

pub(crate) async fn mock_wallet_db() -> WalletDB {
    let wallet_credential = mock_wallet_credential();

    let db = WalletDB::init(&wallet_credential, true).unwrap();

    db
}

pub(crate) fn mock_coin_record(value: u64) -> CoinRecord {
    let hasher = MiMC::new();

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

    let coin_status = CoinStatus::Unused;

    CoinRecord {
        addr_pk,
        addr_sk: ScalarExt::parse_arr(&addr_sk).unwrap(),
        rho: ScalarExt::parse_arr(&rho).unwrap(),
        r: ScalarExt::parse_arr(&r).unwrap(),
        s: ScalarExt::parse_arr(&s).unwrap(),
        v: ScalarExt::parse_arr(&v).unwrap(),
        cm,
        coin_status,
        cm_idx: Some(0),
        coin_idx: Some(0),
        tx_hash: Some("tx_hash".to_string()),
    }
}
