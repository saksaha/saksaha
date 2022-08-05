use std::sync::Arc;

use crate::{rpc::RPC, wallet::Wallet, AppArgs, WalletError};
use log::{error, info};

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(
        self,
        app_args: AppArgs,
    ) -> Result<(), WalletError> {
        println!("wallet main routine start");

        let wallet = {
            let w = Wallet::new();
            Arc::new(w)
        };

        let rpc = RPC::init(app_args.rpc_port, wallet).await?;

        tokio::spawn(async move {
            tokio::join!(rpc.run());

            println!("main process ended");
        });

        let _ = tokio::signal::ctrl_c().await;

        println!("ctrl-c has typed. Terminating process.");

        Ok(())
    }
}
