use crate::{WalletError, APP_NAME};
use colored::Colorize;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_p2p_id::Credential;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(crate) struct WalletCredential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
}

impl WalletCredential {
    pub fn new_random() -> Result<WalletCredential, WalletError> {
        let (sk, pk) = SakKey::generate();
        let secret = sak_crypto::encode_hex(&sk.to_bytes());
        let public_key =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);
        let credential = Credential::new(&secret, &public_key)?;

        let c = WalletCredential {
            public_key: credential.public_key_str,
            secret: credential.secret,
            acc_addr,
        };

        Ok(c)
    }

    pub fn load(
        public_key: String,
        secret: String,
    ) -> Result<WalletCredential, WalletError> {
        let credential = Credential::new(&secret, &public_key)?;
        let acc_addr = SakKey::create_acc_addr(&credential.public_key);

        let c = WalletCredential {
            public_key,
            secret,
            acc_addr,
        };

        Ok(c)
    }

    pub fn persist(&self) -> Result<(), WalletError> {
        let app_path =
            sak_fs::get_app_root_path(APP_NAME)?.join(&self.acc_addr);

        let target_path = app_path.join("CREATED_AT");

        if !target_path.exists() {
            std::fs::create_dir_all(target_path.clone())?;
        }

        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

        std::fs::write(target_path, since_the_epoch.as_millis().to_string())?;

        println!(
            "\nWallet app path is successfully created under {}",
            self.acc_addr.yellow(),
        );

        Ok(())
    }
}
