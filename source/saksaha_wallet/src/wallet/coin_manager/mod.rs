use super::Wallet;
use crate::{db::WalletDB, WalletCredential, WalletError};
use colored::Colorize;
use sak_types::CoinRecord;
use std::sync::Arc;

pub(crate) struct CoinManager {
    coins: Vec<CoinRecord>,
}

impl CoinManager {
    pub async fn init(
        wallet_db: Arc<WalletDB>,
    ) -> Result<CoinManager, WalletError> {
        let coins = wallet_db.schema.get_all_coins()?;

        let coin_count = coins.len();

        println!(
            "\nInitialzing coin manager\n\
            Currently you have {} coins",
            coin_count,
        );

        for (idx, coin) in coins.iter().enumerate() {
            println!(
                "\t- {} {}",
                format!("[{}/{}]", (idx + 1), coin_count).dimmed(),
                coin
            );
        }

        println!("");

        let m = CoinManager { coins };

        Ok(m)
    }

    // pub fn make_coin(&self) {
    //     // for loop
    //     // select the first coin that is not used before.

    //     //
    // }
}
