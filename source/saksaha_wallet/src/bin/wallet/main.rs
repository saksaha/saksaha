mod cli;
mod credential;
mod prompt;

use log::info;
use saksaha_wallet::{App, AppArgs, Config, WalletError};
use std::{thread, time::Duration};

const RUST_LOG_ENV: &str = "
    sak_,
    saksaha_,
    wallet,
";

fn main() -> Result<(), WalletError> {
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args = cli::get_args()?;

    let config = Config::new(cli_args.cfg_profile)?;

    info!("Config created, config: {:?}", config);

    let wallet_credential =
        credential::create_or_get_credential(config.public_key, config.secret)?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        public_key: wallet_credential.public_key,
        secret: wallet_credential.secret,
    };

    let app = App::init();

    app.run(app_args)?;

    Ok(())
}
