use crate::app::prompt;
use crate::credential::WalletCredential;
use crate::db::WalletDB;
use crate::{rpc::RPC, wallet::Wallet, AppArgs, WalletError};
use colored::Colorize;
use log::{error, info};
use std::io::BufRead;
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(
        self,
        app_args: AppArgs,
    ) -> Result<(), WalletError> {
        info!("Wallet main routine starts, app_args: {:?}", app_args);

        let credential =
            create_or_get_credential(app_args.public_key, app_args.secret)?;

        let wallet_db = WalletDB::init(&credential)?;

        let wallet = {
            let w = Wallet::init(credential, wallet_db).await?;

            Arc::new(w)
        };

        let rpc = RPC::init(app_args.rpc_port, wallet).await?;

        tokio::spawn(async move {
            tokio::join!(rpc.run());
        });

        let _ = tokio::signal::ctrl_c().await;

        info!("ctrl-c has typed. Terminating process.");

        Ok(())
    }
}

fn create_or_get_credential(
    public_key: Option<String>,
    secret: Option<String>,
) -> Result<WalletCredential, WalletError> {
    let public_key = public_key;
    let secret = secret;

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

        WalletCredential::load(public_key, secret)?
    };

    Ok(c)
}
