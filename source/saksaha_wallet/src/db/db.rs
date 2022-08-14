use super::WalletDBSchema;
use crate::{credential::WalletCredential, WalletError, APP_NAME};
use log::info;
use sak_kv_db::{KeyValueDatabase, Options};
use std::fs;

pub(crate) struct WalletDB {
    pub(crate) schema: WalletDBSchema,
}

impl WalletDB {
    pub(crate) fn init(
        credential: &WalletCredential,
    ) -> Result<WalletDB, WalletError> {
        let wallet_db_path = {
            let app_path =
                sak_fs::create_or_get_app_path(APP_NAME, &credential.acc_addr)?;

            let db_path = { app_path.join("db") };

            fs::create_dir(db_path.clone())?;

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
}
