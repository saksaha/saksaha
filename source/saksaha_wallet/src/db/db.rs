use super::WalletDBSchema;
use crate::{
    credential::WalletCredential, wallet::CoinManager, WalletError, APP_NAME,
};
use log::info;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_kv_db::{KeyValueDatabase, Options};
use sak_types::CoinStatus;
use sak_types::Sn;
use sak_types::{Cm, CmIdx, CoinRecord};
use std::{borrow::BorrowMut, collections::HashMap, sync::Arc, time::Duration};
use std::{fs, path::PathBuf};
use tokio::sync::RwLockWriteGuard;

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
        saksaha_endpoint: &String,
        coins: &Vec<CoinRecord>,
    ) -> Result<Vec<Sn>, WalletError> {
        let mut old_coin_sn_vec = Vec::<Sn>::new();
        let mut ledger_cms = vec![];

        for coin in coins {
            if coin.is_unconfirmed() {
                let resp = match &coin.tx_hash {
                    Some(tx_hash) => {
                        if !ledger_cms.contains(&tx_hash) {
                            let res = saksaha::get_tx(
                                saksaha_endpoint.clone(),
                                tx_hash.clone(),
                            )
                            .await?
                            .result;

                            ledger_cms.push(tx_hash);

                            res
                        } else {
                            continue;
                        }
                    }
                    None => {
                        return Err(format!(
                            // "No tx_hash has been found in cm: {:?}",
                            // coin.cm
                            "coin (cm: {:?}) does not have tx_hash",
                            coin.cm
                        )
                        .into());
                    }
                };

                match resp {
                    Some(response) => {
                        if let Some(tx) = response.tx {
                            let sns = tx.get_sns();
                            for sn in sns {
                                old_coin_sn_vec.push(sn);
                            }

                            for (cmidx, cm) in tx.get_cm_pairs() {
                                let cm_array = &ScalarExt::parse_arr(&cm)?;

                                self.schema.raw.put_cm_idx(cm_array, &cmidx)?;

                                self.schema.raw.put_coin_status(
                                    cm_array,
                                    &CoinStatus::Unused,
                                )?;
                            }
                        };
                    }
                    None => {
                        println!(
                            "No response with get_tx, {:?}",
                            &coin.tx_hash
                        );
                    } // return Err("No response with get_tx".into()),
                }
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
            if let Some(CoinStatus::Unused) =
                self.schema.raw.get_coin_status(&coin.cm)?
            {
                {
                    let sn = coin.compute_sn();

                    if old_coin_sn_vec.contains(&sn) {
                        self.schema
                            .raw
                            .put_coin_status(&coin.cm, &CoinStatus::Used)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn update_coin_status_unused_to_unconfirmed(
        &self,
        coin: &mut CoinRecord,
    ) -> Result<(), WalletError> {
        if coin.coin_status == CoinStatus::Unused {
            self.schema
                .raw
                .put_coin_status(&coin.cm, &CoinStatus::Unconfirmed)?;

            coin.set_coin_status(CoinStatus::Unconfirmed);
        }

        Ok(())
    }

    pub async fn update_coin_status_to_failed(
        &self,
        coin: &mut CoinRecord,
    ) -> Result<(), WalletError> {
        self.schema
            .raw
            .put_coin_status(&coin.cm, &CoinStatus::Failed)?;

        coin.set_coin_status(CoinStatus::Failed);

        Ok(())
    }
}
