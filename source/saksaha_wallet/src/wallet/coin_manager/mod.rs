use crate::{db::WalletDB, WalletCredential, WalletError};

pub(crate) struct CoinManager {}

impl CoinManager {
    pub async fn init(
        wallet_db: &WalletDB,
    ) -> Result<CoinManager, WalletError> {
        let coins = wallet_db.schema.get_all_coins()?;

        println!("power: [{}] {:?}", coins.len(), coins);

        let m = CoinManager {};

        Ok(m)
    }
}
