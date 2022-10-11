use crate::{fs::SaksahaWalletFS, WalletError};
use colored::Colorize;
use sak_credential::Credential;
use sak_crypto::{SakKey, ToEncodedPoint};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletCredential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialReceipt {
    pub created_at: String,
    pub public_key: String,
    pub acc_addr: String,
}

impl WalletCredential {
    pub fn new_random() -> Result<WalletCredential, WalletError> {
        let (sk, pk) = SakKey::generate();
        let secret = sak_crypto::encode_hex(&sk.to_bytes());
        let public_key = sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);
        let credential = Credential::new(&secret, &public_key)?;

        let c = WalletCredential {
            public_key: credential.public_key_str,
            secret: credential.secret,
            acc_addr,
        };

        Ok(c)
    }

    pub fn load(public_key: &String, secret: &String) -> Result<WalletCredential, WalletError> {
        let credential = Credential::new(&secret, &public_key)?;
        let acc_addr = SakKey::create_acc_addr(&credential.public_key);

        let c = WalletCredential {
            public_key: public_key.to_string(),
            secret: secret.to_string(),
            acc_addr,
        };

        Ok(c)
    }

    pub fn persist(&self) -> Result<(), WalletError> {
        let acc_dir = SaksahaWalletFS::acc_dir(&self.acc_addr)?;

        if !acc_dir.exists() {
            std::fs::create_dir_all(acc_dir.clone())?;
        } else {
            return Err(format!("Credential has already been created").into());
        }

        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

        let receipt = CredentialReceipt {
            created_at: since_the_epoch.as_millis().to_string(),
            public_key: self.public_key.clone(),
            acc_addr: self.acc_addr.clone(),
        };

        let receipt_path = acc_dir.join("account.json");

        std::fs::write(receipt_path, serde_json::to_string_pretty(&receipt)?)?;

        println!(
            "\nWallet app path is successfully created under {}",
            self.acc_addr.yellow(),
        );

        Ok(())
    }
}
