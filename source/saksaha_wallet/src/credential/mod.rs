use crate::WalletError;
use colored::Colorize;
use sak_crypto::{SakKey, ToEncodedPoint};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(crate) struct WalletCredential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
}

const SAKSAHA_WALLET: &'static str = "saksaha-wallet";

impl WalletCredential {
    pub fn new_random() -> WalletCredential {
        let (sk, pk) = SakKey::generate();

        let secret = sak_crypto::encode_hex(&sk.to_bytes());

        let public_key =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let c = WalletCredential {
            public_key,
            secret,
            acc_addr,
        };

        c
    }

    pub fn load(
        public_key: Option<String>,
        secret: Option<String>,
    ) -> Result<WalletCredential, WalletError> {
        let (sk, pk) = SakKey::generate();

        // let public_key = public_key.ok_or("Public key should be provided")?;
        // let secret = secret.ok_or("Secret should be provided")?;

        let secret = sak_crypto::encode_hex(&sk.to_bytes());

        let public_key =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let c = WalletCredential {
            public_key,
            secret,
            acc_addr,
        };

        Ok(c)
    }

    pub fn persist(&self) -> Result<(), WalletError> {
        let app_path =
            sak_fs::create_or_get_app_path(SAKSAHA_WALLET, &self.acc_addr)?;

        let target_path = app_path.join("CREATED_AT");
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

        std::fs::write(target_path, since_the_epoch.as_millis().to_string())?;

        println!(
            "\nWallet app path is successfully created under {}",
            self.acc_addr.yellow(),
        );

        Ok(())
    }
}
