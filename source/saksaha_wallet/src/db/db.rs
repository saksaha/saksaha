use log::info;
use sak_kv_db::{KeyValueDatabase, Options};

use crate::app::WalletError;

use super::WalletDBSchema;

pub(crate) struct WalletDB {
    pub(crate) schema: WalletDBSchema,
}

impl WalletDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<WalletDB, WalletError> {
        let wallet_db_path = {
            let app_path = sak_fs::create_or_get_app_path_wallet(app_prefix)?;
            let db_path = { app_path.join("db") };

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

    // pub(crate) async fn register_user(
    //     &self,
    //     user_id: &String,
    // ) -> Result<(), WalletError> {
    //     match self.schema.get_my_sk(user_id).await? {
    //         Some(_) => {
    //             warn!("user_id already exists");
    //             return Ok(());
    //         }
    //         None => (),
    //     };

    //     let (secret_str, public_key_str, sig_str) = {
    //         let (sk, pk) = SakKey::generate();
    //         let secret_str = sak_crypto::encode_hex(&sk.to_bytes());

    //         let public_key_str =
    //             sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

    //         let sig_str = {
    //             let sign_key = SigningKey::from(&sk);
    //             let sign_key_vec = sign_key.to_bytes().to_vec();
    //             match serde_json::to_string(&sign_key_vec) {
    //                 Ok(str) => str,
    //                 Err(err) => {
    //                     return Err(format!(
    //                         "Failed to change vec to string, err: {}",
    //                         err
    //                     )
    //                     .into());
    //                 }
    //             }
    //         };
    //         (secret_str, public_key_str, sig_str)
    //     };

    //     self.schema
    //         .put_user_data(user_id, &secret_str, &public_key_str, &sig_str)
    //         .await?;
    //     Ok(())
    // }
}
