use crate::{db::WalletDB, CredentialManager, WalletError};
use log::debug;
use sak_types::CoinRecord;

use super::CoinManager;

pub(crate) struct Wallet {
    wallet_db: WalletDB,
    credential_manager: CredentialManager,
}

impl Wallet {
    pub async fn init(
        credential_manager: CredentialManager,
        wallet_db: WalletDB,
    ) -> Result<Wallet, WalletError> {
        let coin_manager = CoinManager::new();

        let wallet = Wallet {
            wallet_db,
            credential_manager,
        };

        // for development
        init_for_dev(&wallet).await?;

        Ok(wallet)
    }

    #[inline]
    pub fn get_db(&self) -> &WalletDB {
        &self.wallet_db
    }

    #[inline]
    pub fn get_credential_manager(&self) -> &CredentialManager {
        &self.credential_manager
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
async fn init_for_dev(wallet: &Wallet) -> Result<(), WalletError> {
    {
        let value = 100;

        let coin = CoinRecord::new(0x11, 0x12, 0x13, 0x14, value, None)?;

        debug!("[demo coin: user_1] {:#?}", coin);

        wallet.get_db().schema.put_coin(&coin)?;
    }

    // {
    //     let value = 100;

    //     let coin = CoinRecord::new(0x21, 0x22, 0x23, 0x24, value, None)?;

    //     debug!("[demo coin: user_2] {:#?}", coin);

    //     wallet.apis.db.schema.put_coin(&coin)?;
    // }

    Ok(())
}
