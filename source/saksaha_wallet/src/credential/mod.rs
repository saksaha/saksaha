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
        println!("0");

        let app_path =
            sak_fs::create_or_get_app_path(APP_NAME)?.join(&self.acc_addr);

        println!("1");

        let target_path = app_path.join("CREATED_AT");
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

        std::fs::write(target_path, since_the_epoch.as_millis().to_string())?;

        println!("2");

        println!(
            "\nWallet app path is successfully created under {}",
            self.acc_addr.yellow(),
        );

        Ok(())
    }
}
