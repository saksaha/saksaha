mod cli;

use envelope_term::{App, AppArgs, Config, EnvelopeError};

fn main() -> Result<(), EnvelopeError> {
    let cli_args = cli::get_args()?;

    let config = Config::new(&cli_args.cfg_profile)?;

    let app_args = AppArgs { config };

    let app = App {};

    app.run(app_args)?;

    Ok(())
}
