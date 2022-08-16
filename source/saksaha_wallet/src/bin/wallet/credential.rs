use crate::prompt;
use colored::Colorize;
use log::info;
use saksaha_wallet::{WalletCredential, WalletError};

pub fn create_or_get_credential(
    public_key: Option<String>,
    secret: Option<String>,
) -> Result<WalletCredential, WalletError> {
    let c = if public_key.is_none() || secret.is_none() {
        let _ = prompt::run()?;

        let c = WalletCredential::new_random()?;

        println!(
            "\n{} created! \nWe recommend that you write \n\
            this down to a safe location only you may know. \n\
            Once lost, this information cannot be retrieved, forever.",
            "Credential".yellow(),
        );

        println!(
            "\n{}: {} \n{}: {} \n{}: {}",
            "Public key".cyan(),
            c.public_key,
            "Secret".cyan(),
            c.secret,
            "Account address".cyan(),
            c.acc_addr,
        );

        c.persist()?;

        c
    } else {
        let public_key = public_key.ok_or("Public key should be provided")?;
        let secret = secret.ok_or("Secret should be provided")?;

        let w = WalletCredential::load(public_key, secret)?;

        info!(
            "Wallet credential has been successfully loaded, acc_addr: {}",
            w.acc_addr.yellow(),
        );

        w
    };

    Ok(c)
}
