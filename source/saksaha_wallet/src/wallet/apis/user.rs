use super::WalletApis;
use crate::{Wallet, WalletError};
use log::warn;
use sak_crypto::{
    PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};

impl WalletApis {
    pub(crate) async fn register_user(
        &self,
        user_id: &String,
    ) -> Result<(), WalletError> {
        match self.db.schema.get_my_sk(user_id)? {
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

        self.db
            .schema
            .put_user_data(user_id, &secret_str, &public_key_str, &sig_str)
            .await?;

        Ok(())
    }
}
