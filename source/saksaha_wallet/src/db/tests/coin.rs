use super::utils;
use sak_logger::SakLogger;

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_db_store_coin_record() {
    SakLogger::init_test_console().unwrap();

    let db = utils::mock_wallet_db().await;

    let coin = utils::mock_coin_record(100);

    db.schema.put_coin(&coin).unwrap();

    let coin_from_db = db.schema.get_coin(&coin.cm).unwrap();

    assert_eq!(coin.addr_pk, coin_from_db.addr_pk);

    assert_eq!(coin.addr_sk, coin_from_db.addr_sk);

    assert_eq!(coin.rho, coin_from_db.rho);

    assert_eq!(coin.r, coin_from_db.r);

    assert_eq!(coin.s, coin_from_db.s);

    assert_eq!(coin.v, coin_from_db.v);

    assert_eq!(coin.cm, coin_from_db.cm);

    assert_eq!(coin.cm_idx, coin_from_db.cm_idx);

    assert_eq!(coin.coin_idx, coin_from_db.coin_idx);

    assert_eq!(coin.tx_hash, coin_from_db.tx_hash);
}
