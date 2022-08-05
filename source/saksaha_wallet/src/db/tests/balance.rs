use crate::db::WalletDB;

async fn make_dummy_wallet_db() -> WalletDB {
    WalletDB::init(&"test".to_string()).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_wallet_get_all_coins() {

    // TODO fix!
    // let db = WalletDB::init(&test_string).await.unwrap();

    // db.register_user(&USER_1.to_string()).await.unwrap();

    // match db.schema.get_my_sk(&USER_1.to_string()).await.unwrap() {
    //     Some(s) => s,
    //     None => {
    //         panic!("no matching secret key with user : {:?}", USER_1)
    //     }
    // };
}
