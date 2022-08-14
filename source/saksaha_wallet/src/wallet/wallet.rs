use super::apis::{self, WalletApis};
use crate::{
    credential::WalletCredential, db::WalletDB, CredentialManager, WalletError,
};
use futures::sink::Send;
use log::debug;
use sak_crypto::{Hasher, ScalarExt};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::{CoinRecord, CoinStatus};
use type_extension::U8Array;

pub(crate) struct Wallet {
    apis: WalletApis,
}

impl Wallet {
    pub async fn init(
        credential_manager: CredentialManager,
        wallet_db: WalletDB,
    ) -> Result<Wallet, WalletError> {
        let apis = WalletApis {
            db: wallet_db,
            credential_manager,
        };

        let w = Wallet { apis };

        // for development
        init_for_demo(&w).await?;

        Ok(w)
    }

    #[inline]
    pub fn get_apis(&self) -> &WalletApis {
        &self.apis
    }
}

// pub struct SendTxPourRequest {
//     pi: U8Array,
//     sn_1: U8Array,
//     sn_2: U8Array,
//     cm_1: U8Array,
//     cm_2: U8Array,
//     merkle_rt: U8Array,
// }

async fn init_for_demo(wallet: &Wallet) -> Result<(), WalletError> {
    {
        let value = 100;

        let coin = CoinRecord::new(0x11, 0x12, 0x13, 0x14, value, None)?;

        debug!("[demo coin: user_1] {:#?}", coin);

        wallet.apis.db.schema.put_coin(&coin)?;
    }

    {
        let value = 100;

        let coin = CoinRecord::new(0x21, 0x22, 0x23, 0x24, value, None)?;

        debug!("[demo coin: user_2] {:#?}", coin);

        wallet.apis.db.schema.put_coin(&coin)?;
    }

    Ok(())
}
