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

    // pub fn set_coins(&self, coins: &Vec<CoinRecord>) -> Self {
    //     let coins = *coins;
    //     CoinManager { coins }
    // }

    pub fn get_next_available_coin(&self) -> Option<&CoinRecord> {
        let vec_coins = &self.coins;

        println!("\t[+] Available coin count: {:?}", vec_coins.len());

        for coin in vec_coins {
            if coin.coin_status == CoinStatus::Unused {
                return Some(&coin);
            }
        }

        return None;
    }

    // pub fn make_coin(&self) {
    //     // for loop
    //     // select the first coin that is not used before.

    //     //
    // }

    pub fn update_coin(
        &mut self,
        coin_record: CoinRecord,
    ) -> Result<(), WalletError> {
        println!("coin manager has been updated");

        self.coins.push(coin_record);

        Ok(())
    }
}
