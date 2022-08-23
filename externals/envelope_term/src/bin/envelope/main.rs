mod cli;

use envelope_term::{AppArgs, Config, EnvelopeError};

fn main() -> Result<(), EnvelopeError> {
    let cli_args = cli::get_args()?;

    let config = Config::new(&cli_args.cfg_profile)?;

    let app_args = AppArgs { config };

    envelope_term::run_app(app_args)?;

    Ok(())
}
