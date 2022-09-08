use crate::{db::WalletDB, WalletError};
use colored::Colorize;
use sak_types::CoinRecord;
use sak_types::CoinStatus;
use std::sync::Arc;

pub(crate) struct CoinManager {
    pub(crate) coins: Vec<CoinRecord>,
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

        let m = CoinManager { coins };

        Ok(m)
    }

    pub fn get_next_available_coin(&mut self) -> Option<&mut CoinRecord> {
        for coin in self.coins.iter_mut() {
            if !coin.has_zero_value() && coin.is_unused() {
                return Some(coin);
            }
        }

        None
    }

    pub fn put_coin(
        &mut self,
        coin_record: CoinRecord,
    ) -> Result<(), WalletError> {
        self.coins.push(coin_record);

        Ok(())
    }

    pub fn get_all_coins(&self) -> Result<Vec<CoinRecord>, WalletError> {
        Ok(self.coins.clone())
    }
}
