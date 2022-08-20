use super::WalletDBSchema;
use crate::{credential::WalletCredential, WalletError, APP_NAME};
use log::info;
use sak_kv_db::{KeyValueDatabase, Options};
use sak_types::CoinRecord;
use sak_types::CoinStatus;
use sak_types::Sn;
use std::{fs, path::PathBuf};

pub(crate) struct WalletDB {
    pub(crate) schema: WalletDBSchema,
}

impl WalletDB {
    pub(crate) fn init(
        credential: &WalletCredential,
        force_reset: bool,
    ) -> Result<WalletDB, WalletError> {
        if force_reset {
            let db_path = Self::get_db_path(&credential.acc_addr)?;

            info!(
                "'Force reset' is on. Removing db path if exists, \
                db_path: {:?}",
                db_path,
            );

            if db_path.exists() {
                std::fs::remove_dir_all(db_path)?;
            }
        }

        let wallet_db_path = {
            let db_path = Self::get_db_path(&credential.acc_addr)?;

            if !db_path.exists() {
                fs::create_dir_all(db_path.clone())?;
            }

            db_path
        };

        let a = wallet_db_path.join("p");

        fs::write(a, "power")?;

        info!("Wallet db path: {:?}", wallet_db_path);

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(
            wallet_db_path,
            options,
            WalletDBSchema::make_cf_descriptors(),
        ) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "Error initializing key value database, err: {}",
                    err
                )
                .into());
            }
        };

        let schema = WalletDBSchema::new(kv_db.db_instance);

        let wallet_db = WalletDB { schema };

        Ok(wallet_db)
    }

    pub fn get_db_path(acc_addr: &String) -> Result<PathBuf, WalletError> {
        let app_path =
            sak_fs::create_or_get_app_path(APP_NAME)?.join(&acc_addr);

        let db_path = app_path.join("db");

        Ok(db_path)
    }

    pub async fn update_coin_status_unconfirmed_to_unused(
        &self,
        coins: &Vec<CoinRecord>,
    ) -> Result<Vec<Sn>, WalletError> {
        let mut old_coin_sn_vec = Vec::<Sn>::new();

        println!("update");

        for coin in coins {
            match coin.coin_status.clone() {
                CoinStatus::Unconfirmed => {
                    // get tx_hash related with coin from db or itself

                    // send req to Node
                    let resp = saksaha::get_tx(tx_hash.clone())
                        .await?
                        .result
                        .ok_or("json_response error")?;

                    if let Some(tx) = resp.tx {
                        // insert `sn` to Vec<Sn> for
                        // updating status of old_coin status later
                        old_coin_sn_vec.push(tx.get_sn());

                        self.schema.raw.single_put_coin_status(
                            &coin.cm,
                            &CoinStatus::Unused,
                        )?;

                        // coin_manager_lock.make_coin_status_unused(coin)?;

                        // coin.make_status_used();

                        // wallet_db.schema.raw.single_put_coin_status(
                        //     &coin.cm,
                        //     &CoinStatus::Unused,
                        // )?;
                    };
                }

                CoinStatus::Used => {}

                CoinStatus::Unused => {}
            }
        }

        Ok(old_coin_sn_vec)
    }

    pub async fn update_coin_status_unused_to_used(
        &self,
        old_coin_sn_vec: Vec<Sn>,
        coins: &Vec<CoinRecord>,
    ) -> Result<(), WalletError> {
        for coin in coins {
            match coin.coin_status.clone() {
                CoinStatus::Unconfirmed => {}

                CoinStatus::Used => {}

                CoinStatus::Unused => {
                    println!("\t[+] CoinStatus update! [Unused] -> [Used]");

                    let sn = coin.compute_sn();

                    if old_coin_sn_vec.contains(&sn) {
                        self.schema.raw.single_put_coin_status(
                            &coin.cm,
                            &CoinStatus::Used,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
