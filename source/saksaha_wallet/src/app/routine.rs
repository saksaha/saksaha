use crate::credential::CredentialManager;
use crate::db::WalletDB;
use crate::{rpc::RPC, wallet::Wallet, AppArgs, WalletError};
use log::{error, info};
use std::sync::Arc;

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(
        self,
        app_args: AppArgs,
    ) -> Result<(), WalletError> {
        info!("Wallet main routine starts, app_args: {:?}", app_args);

        let credential_manager =
            CredentialManager::init(app_args.wallet_credential)?;

        let wallet_db =
            WalletDB::init(&credential_manager.get_credential(), false)?;

        let wallet = {
            let w =
                Wallet::init(credential_manager, wallet_db, app_args.config)
                    .await?;

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
