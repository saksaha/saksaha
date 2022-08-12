use crate::{
    credential::Credential, rpc::RPC, wallet::Wallet, AppArgs, WalletError,
};
use log::{error, info};
use std::io::BufRead;
use std::sync::Arc;
use std::time::Duration;

const APP_PREFIX: &'static str = "default";

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(
        self,
        app_args: AppArgs,
    ) -> Result<(), WalletError> {
        info!("Wallet main routine starts, app_args: {:?}", app_args);

        use std::io::BufRead;
        let stdin = std::io::stdin();

        loop {
            println!("11l");

            for line in stdin.lock().lines() {
                println!("44, {}", line.unwrap());
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        println!("should be waiting!!! 123123");

        {
            let public_key = app_args.public_key;
            let secret = app_args.secret;

            if public_key.is_none() || secret.is_none() {
                info!(
                    "Either public_key or secret is empty. Would you want to\
                    we proceed to create a new credential?",
                );

                // let mut buffer = String::new();
                let stdin = std::io::stdin();

                for line in stdin.lock().lines() {
                    println!("opwer: {:?}", line.unwrap());
                }

                println!("123123");
            }
        }

        // let app_prefix = app_args.app_prefix.unwrap_or_else(|| {
        //     info!("App prefix is not specified, defaults to '{}'", APP_PREFIX);

        //     APP_PREFIX.to_string()
        // });

        // let credential = Credential::new(app_args.id, app_args.key);

        // let wallet = {
        //     let w = Wallet::init(app_prefix, credential).await?;

        //     Arc::new(w)
        // };

        // let rpc = RPC::init(app_args.rpc_port, wallet).await?;

        // tokio::spawn(async move {
        //     tokio::join!(rpc.run());
        // });

        let _ = tokio::signal::ctrl_c().await;

        info!("ctrl-c has typed. Terminating process.");

        Ok(())
    }
}
