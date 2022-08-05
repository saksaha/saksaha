mod cli;

use saksaha_wallet::{App, AppArgs, WalletError};

const RUST_LOG_ENV: &str = "
    sak_,
    saksaha
";

fn main() -> Result<(), WalletError> {
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args = cli::get_args()?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        app_prefix: cli_args.app_prefix,
        id: cli_args.id,
        key: cli_args.key,
    };

    let app = App::init();

    app.run(app_args)?;

    Ok(())
}
