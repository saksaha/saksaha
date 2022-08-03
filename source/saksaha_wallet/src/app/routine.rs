use crate::{rpc::RPC, WalletError};
use log::{error, info};

pub(crate) struct Routine {}

impl Routine {
    pub(crate) async fn run(&self) -> Result<(), WalletError> {
        println!("wallet main routine start");

        let rpc = RPC::init();

        tokio::spawn(async move {
            tokio::join!(rpc.run(),);

            println!("main process ended");
        });

        let _ = tokio::signal::ctrl_c().await;

        println!("ctrl-c has typed. Terminating process.");

        Ok(())
    }
}
