use crate::{
    credential::Credential, rpc::RPC, wallet::Wallet, AppArgs, WalletError,
};
use log::{error, info};
use std::sync::Arc;

const APP_PREFIX: &'static str = "default";

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(
        self,
        app_args: AppArgs,
    ) -> Result<(), WalletError> {
        info!("Wallet main routine starts, app_args: {:?}", app_args);

        let app_prefix = app_args.app_prefix.unwrap_or_else(|| {
            info!("App prefix is not specified, defaults to '{}'", APP_PREFIX);

            APP_PREFIX.to_string()
        });

        let credential = Credential::new(app_args.id, app_args.key);

        let wallet = {
            let w = Wallet::init(app_prefix, credential).await?;

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
