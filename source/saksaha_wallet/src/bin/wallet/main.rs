mod cli;
mod credential;
mod prompt;

use cli::CLIArgs;
use sak_logger::info;
use sak_logger::SakLogger;
use sak_logger::RUST_LOG_ENV;
use saksaha_wallet::{App, AppArgs, Config, WalletError};

fn main() -> Result<(), WalletError> {
    let cli_args = cli::get_args()?;

    let config = make_config(&cli_args)?;

    println!("Config created, config: {:?}", config);

    let wallet_credential =
        credential::create_or_get_credential(&config.public_key, &config.secret)?;

    // {
    //     if std::env::var("RUST_LOG").is_err() {
    //         std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    //     }

    //     let public_key = &config.p2p.public_key_str;
    //     let log_root_dir = SaksahaWallet::config_dir()?;
    //     let _ = SakLogger::init(&log_root_dir, public_key.as_str())?;
    // }

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
