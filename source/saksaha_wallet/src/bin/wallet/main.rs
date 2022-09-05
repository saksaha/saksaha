mod cli;
mod credential;
mod prompt;

use cli::CLIArgs;
use log::info;
use sak_logger::RUST_LOG_ENV;
use saksaha_wallet::{App, AppArgs, Config, WalletError};

fn main() -> Result<(), WalletError> {
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args = cli::get_args()?;

    let config = make_config(&cli_args)?;

    info!("Config created, config: {:?}", config);

    let wallet_credential = credential::create_or_get_credential(
        &config.public_key,
        &config.secret,
    )?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        wallet_credential,
        config,
    };

    let app = App::init();

    app.run(app_args)?;

    Ok(())
}

fn make_config(cli_args: &CLIArgs) -> Result<Config, WalletError> {
    let mut config = Config::new(&cli_args.cfg_profile)?;

    if let Some(endpoint) = &cli_args.saksaha_endpoint {
        config.saksaha_endpoint = Some(endpoint.to_string());
    }

    Ok(config)
}
