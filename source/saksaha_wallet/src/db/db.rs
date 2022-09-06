use super::WalletDBSchema;
use crate::{
    credential::WalletCredential, wallet::CoinManager, WalletError, APP_NAME,
};
use log::info;
use sak_crypto::ScalarExt;
use sak_kv_db::{KeyValueDatabase, Options};
use sak_types::CoinRecord;
use sak_types::CoinStatus;
use sak_types::Sn;
use std::{borrow::BorrowMut, sync::Arc, time::Duration};
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

        for coin in coins {
            if coin.coin_status == CoinStatus::Unconfirmed {
                let resp = match &coin.tx_hash {
                    Some(tx_hash) => {
                        saksaha::get_tx(
                            saksaha_endpoint.clone(),
                            tx_hash.clone(),
                        )
                        .await?
                        .result
                    }
                    None => {
                        return Err(format!(
                            "No tx_hash has been found in cm: {:?}",
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

                            self.schema.raw.put_coin_status(
                                &coin.cm,
                                &CoinStatus::Unused,
                            )?;

                            {
                                let cm_idx_base = tx
                                    .get_cm_pairs()
                                    .get(0)
                                    .ok_or("expect (CmIdx, Cm)")?
                                    .0;

                                let cm_idx_offset = coin
                                    .cm_idx
                                    .ok_or("expect cm_idx_offset")?;

                                let cm_idx = cm_idx_base + cm_idx_offset;

                                self.schema
                                    .raw
                                    .put_cm_idx(&coin.cm, &cm_idx)?;
                            }
                        };
                    }
                    None => return Err("No response with get_tx".into()),
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
            // match coin.coin_status {
            match self
                .schema
                .raw
                .get_coin_status(&coin.cm)?
                .unwrap_or(CoinStatus::Unconfirmed)
            {
                CoinStatus::Unused => {
                    let sn = coin.compute_sn();

                    if old_coin_sn_vec.contains(&sn) {
                        self.schema
                            .raw
                            .put_coin_status(&coin.cm, &CoinStatus::Used)?;
                    }
                }
                _ => {}
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

            coin.set_coin_status_to(CoinStatus::Unconfirmed);
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

        coin.set_coin_status_to(CoinStatus::Failed);

        Ok(())
    }
}
