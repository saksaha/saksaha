use crate::{credential::WalletCredential, db::WalletDB, rpc::RPC, Wallet};
use std::sync::Arc;

pub(crate) struct TestContext {
    pub rpc: RPC,
}

pub(crate) async fn make_test_context() -> TestContext {
    let credential = {
        let public_key = String::from(
            "043fd721eba5004dad3733ddf54638e8d9a5b4d6ad05dcf9860b95bfb\
            8faf5e341e6c4c492d6eb649a83e9c4766252697da85c601136e9bfe65\
            fa6531eb136bfb3",
        );

        let secret = String::from(
            "3755bfcd0c954a4c53d6a0878806140c865\
            160cf9db3a22c04ca6cea627a37f1",
        );

        WalletCredential::load(public_key, secret).unwrap()
    };

    let wallet_db = WalletDB::init(&credential).unwrap();

    let wallet = {
        let w = Wallet::init(credential, wallet_db).await.unwrap();

        Arc::new(w)
    };

    let rpc = RPC::init(None, wallet).await.unwrap();

    TestContext { rpc }
}
