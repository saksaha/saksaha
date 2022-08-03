use crate::{db::EnvelopeDBSchema, WalletError};
use log::{info, warn};
use sak_crypto::{
    PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};
use sak_kv_db::{KeyValueDatabase, Options};

pub(crate) struct EnvelopeDB {
    pub(crate) schema: EnvelopeDBSchema,
}

impl EnvelopeDB {
    pub(crate) async fn init(
        app_prefix: &String,
    ) -> Result<EnvelopeDB, WalletError> {
        let envelope_db_path = {
            let app_path = sak_fs::create_or_get_app_path_evl(app_prefix)?;
            let db_path = { app_path.join("db") };

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(
            envelope_db_path,
            options,
            EnvelopeDBSchema::make_cf_descriptors(),
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

        let schema = EnvelopeDBSchema::new(kv_db.db_instance);

        let database = EnvelopeDB { schema };

        info!("Initialized Database");

        Ok(database)
    }

    pub(crate) async fn register_user(
        &self,
        user_id: &String,
    ) -> Result<(), WalletError> {
        match self.schema.get_my_sk(user_id).await? {
            Some(_) => {
                warn!("user_id already exists");
                return Ok(());
            }
            None => (),
        };

        let (secret_str, public_key_str, sig_str) = {
            let (sk, pk) = SakKey::generate();
            let secret_str = sak_crypto::encode_hex(&sk.to_bytes());

            let public_key_str =
                sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

            let sig_str = {
                let sign_key = SigningKey::from(&sk);
                let sign_key_vec = sign_key.to_bytes().to_vec();
                match serde_json::to_string(&sign_key_vec) {
                    Ok(str) => str,
                    Err(err) => {
                        return Err(format!(
                            "Failed to change vec to string, err: {}",
                            err
                        )
                        .into());
                    }
                }
            };
            (secret_str, public_key_str, sig_str)
        };

        self.schema
            .put_user_data(user_id, &secret_str, &public_key_str, &sig_str)
            .await?;

        Ok(())
    }
}
