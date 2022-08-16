use super::CoinManager;
use crate::{db::WalletDB, Config, CredentialManager, WalletError};
use colored::Colorize;
use log::debug;
use sak_types::CoinRecord;

pub(crate) struct Wallet {
    wallet_db: WalletDB,
    credential_manager: CredentialManager,
}

impl Wallet {
    pub async fn init(
        credential_manager: CredentialManager,
        wallet_db: WalletDB,
        config: Config,
    ) -> Result<Wallet, WalletError> {
        let wallet = Wallet {
            wallet_db,
            credential_manager,
        };

        bootstrap_wallet(&wallet, config).await?;

        let coin_manager = CoinManager::init(&wallet).await?;

        // for development
        // init_for_dev(&wallet).await?;

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

        for (idx, coin) in coin_records.iter().enumerate() {
            let res = wallet.get_db().schema.put_coin(&coin);

            match res {
                Ok(r) => {
                    println!(
                        "\t[{}/{}] Bootstrapped a coin, cm: {}, val: {}",
                        idx, coin_count, coin.cm, coin.v
                    );
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

async fn init_for_dev(wallet: &Wallet) -> Result<(), WalletError> {
    // {
    //     let value = 100;

    //     let coin = CoinRecord::new(0x11, 0x12, 0x13, 0x14, value, None)?;

    //     coin.cm;

    //     debug!("[demo coin: user_1] {:#?}", coin);

    //     wallet.get_db().schema.put_coin(&coin)?;
    // }

    // {
    //     let value = 100;

    //     let coin = CoinRecord::new(0x21, 0x22, 0x23, 0x24, value, None)?;

    //     debug!("[demo coin: user_2] {:#?}", coin);

    //     wallet.apis.db.schema.put_coin(&coin)?;
    // }

    Ok(())
}
