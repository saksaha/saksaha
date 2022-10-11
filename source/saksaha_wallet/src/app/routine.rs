use crate::credential::CredentialManager;
use crate::db::WalletDB;
use crate::fs::SaksahaWalletFS;
use crate::{rpc::RPC, wallet::Wallet, AppArgs, WalletError};
use sak_logger::RUST_LOG_ENV;
use sak_logger::{info, SakLogger};
use std::sync::Arc;

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(self, app_args: AppArgs) -> Result<(), WalletError> {
        println!("Wallet main routine starts, app_args: {:?}", app_args);

        {
            if std::env::var("RUST_LOG").is_err() {
                std::env::set_var("RUST_LOG", RUST_LOG_ENV);
            }

            let public_key = &app_args.wallet_credential.public_key;
            let log_root_dir = SaksahaWalletFS::config_dir()?;
            let _ = SakLogger::init(&log_root_dir, public_key.as_str())?;
        }

        let rpc_port = app_args.config.rpc_port;

        let wallet = {
            let credential_manager = CredentialManager::init(app_args.wallet_credential)?;

            let wallet_db = WalletDB::init(&credential_manager.get_credential(), false)?;

            let w = Wallet::init(credential_manager, wallet_db, app_args.config).await?;

            Arc::new(w)
        };

        let rpc = RPC::init(rpc_port, wallet).await?;

        tokio::spawn(async move {
            tokio::join!(rpc.run());
        });

        let _ = tokio::signal::ctrl_c().await;

        info!("ctrl-c has typed. Terminating process.");

        Ok(())
    }
}
