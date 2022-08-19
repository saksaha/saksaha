mod cli;

use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{Arg, ArgMatches};
use envelope_term::{run_app, AppArgs, Config, EnvelopeError};
use std::path::PathBuf;

fn main() -> Result<(), EnvelopeError> {
    let cli_args = cli::get_args()?;

    let config = Config::new(&cli_args.cfg_profile)?;

    let app_args = AppArgs { config };

    run_app(app_args)?;

    Ok(())
}
