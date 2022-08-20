use super::CoinManager;
use crate::{db::WalletDB, Config, CredentialManager, WalletError};
use colored::Colorize;
use std::sync::Arc;
use tokio::sync::RwLock;
pub const GAS: u64 = 10;

pub(crate) struct Wallet {
    wallet_db: Arc<WalletDB>,
    credential_manager: CredentialManager,
    coin_manager: RwLock<CoinManager>,
}

impl Wallet {
    pub async fn init(
        credential_manager: CredentialManager,
        wallet_db: WalletDB,
        config: Config,
    ) -> Result<Wallet, WalletError> {
        let wallet_db = Arc::new(wallet_db);

        let coin_manager =
            RwLock::new(CoinManager::init(wallet_db.clone()).await?);

        let wallet = Wallet {
            wallet_db,
            credential_manager,
            coin_manager,
        };

        bootstrap_wallet(&wallet, config).await?;

        Ok(wallet)
    }

    #[inline]
    pub fn get_db(&self) -> &WalletDB {
        &self.wallet_db
    }

    // #[inline]
    // pub fn get_db_mut(&mut self) -> &mut WalletDB {
    //     &mut self.wallet_db
    // }
    #[inline]

    pub fn get_coin_manager(&self) -> &RwLock<CoinManager> {
        &self.coin_manager
    }

    // #[inline]
    // pub fn get_coin_manager_mut(&mut self) -> &mut RwLock<CoinManager> {
    //     &mut self.coin_manager
    // }

    #[inline]
    pub fn get_credential_manager(&self) -> &CredentialManager {
        &self.credential_manager
    }
}

async fn bootstrap_wallet(
    wallet: &Wallet,
    config: Config,
) -> Result<(), WalletError> {
    println!(
        "\n{} wallet\nConfig: {:#?}\n",
        "Bootstrapping".green(),
        config
    );

    if let Some(coin_records) = config.coin_records {
        let coin_count = coin_records.len();

        println!(
            "\nTotal {} coins to bootstrap",
            coin_count.to_string().green()
        );

        for (idx, coin) in coin_records.into_iter().enumerate() {
            let res = wallet.get_db().schema.put_coin(&coin);

            match res {
                Ok(_r) => {
                    println!(
                        "\t[{}/{}] Bootstrapped a coin\n\t\tcm: {}\n\t\tval: {}",
                        idx, coin_count, coin.cm, coin.v
                    );

                    wallet.coin_manager.write().await.put_coin(coin)?;
                }
                Err(err) => {
                    println!(
                        "\t- [{}/{}] Error bootstrapping coin, cm: {}, \n\
                        \terr: {}",
                        idx, coin_count, coin.cm, err,
                    );
                }
            };
        }
    }

    Ok(())
}
