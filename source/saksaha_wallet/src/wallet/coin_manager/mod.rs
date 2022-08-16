use super::Wallet;
use crate::{db::WalletDB, WalletCredential, WalletError};
use sak_types::CoinRecord;

pub(crate) struct CoinManager {
    coins: Vec<CoinRecord>,
}

impl CoinManager {
    pub async fn init(wallet: &Wallet) -> Result<CoinManager, WalletError> {
        let coins = wallet.get_db().schema.get_all_coins()?;

        let coin_count = coins.len();

        println!(
            "\nInitialzing coin manager\n\
            Currently you have {} coins",
            coin_count,
        );

        for (idx, coin) in coins.iter().enumerate() {
            println!("\t- [{}/{}] {}", idx + 1, coin_count, coin);
        }

        let m = CoinManager { coins };

        Ok(m)
    }
}
