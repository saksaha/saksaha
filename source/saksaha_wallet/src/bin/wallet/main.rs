mod cli;
mod credential;
mod prompt;

use saksaha_wallet::{App, AppArgs, WalletError};
use std::{thread, time::Duration};

const RUST_LOG_ENV: &str = "
    sak_,
    saksaha_
";

fn main() -> Result<(), WalletError> {
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args = cli::get_args()?;

    let wallet_credential = credential::create_or_get_credential(
        cli_args.public_key,
        cli_args.secret,
    )?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        public_key: wallet_credential.public_key,
        secret: wallet_credential.secret,
    };

    let app = App::init();

    app.run(app_args)?;

    Ok(())
}
