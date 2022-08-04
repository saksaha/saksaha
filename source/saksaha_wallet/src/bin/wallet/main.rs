mod cli;

use saksaha_wallet::{App, AppArgs, WalletError};

fn main() -> Result<(), WalletError> {
    let cli_args = cli::get_args()?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        app_prefix: cli_args.app_prefix,
    };

    let app = App::init();

    app.run(app_args)?;

    Ok(())
}
