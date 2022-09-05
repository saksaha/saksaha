mod cli;

use cli::CLIArgs;
use envelope_term::{App, AppArgs, Config, EnvelopeError};

fn main() -> Result<(), EnvelopeError> {
    let cli_args = cli::get_args()?;

    let config = make_config(&cli_args)?;

    let app_args = AppArgs { config };

    let app = App {};

    app.run(app_args)?;

    Ok(())
}

fn make_config(cli_args: &CLIArgs) -> Result<Config, EnvelopeError> {
    let mut config = Config::new(&cli_args.cfg_profile)?;

    if let Some(endpoint) = &cli_args.saksaha_endpoint {
        config.saksaha_endpoint = Some(endpoint.to_string());
    }

    Ok(config)
}
