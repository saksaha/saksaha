use super::apis::{self, WalletApis};
use crate::{credential::Credential, db::WalletDB, WalletError};
use futures::sink::Send;
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};

pub(crate) struct Wallet {
    pub apis: WalletApis,
    credential: Credential,
}

impl Wallet {
    pub async fn init(
        app_prefix: String,
        credential: Credential,
    ) -> Result<Wallet, WalletError> {
        let wallet_db = WalletDB::init(&app_prefix).await?;

        let apis = WalletApis { db: wallet_db };

        let w = Wallet { credential, apis };

        Ok(w)
    }
}
