use clap::ArgMatches;
use clap::{arg, command, value_parser, ArgAction, Command};
use envelope_term::term;
use envelope_term::term::TermArgs;
use envelope_term::EnvelopeError;
use std::path::PathBuf;

fn get_cli_args() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --debug "Turn debugging information on"
            )
            .action(ArgAction::Count),
        )
        .subcommand(Command::new("test").about("does testing things").arg(
            arg!(-l --list "lists test values").action(ArgAction::SetTrue),
        ))
        .get_matches();

    matches
}

fn main() -> Result<(), EnvelopeError> {
    let cli_args = get_cli_args();

    let pconfig_path = resolve_pconfig_path();

    let user_prefix = String::from("user_1");
    let term_args = TermArgs {
        pconfig_path,
        user_prefix,
    };

    term::run(term_args)?;

    Ok(())
}

fn resolve_pconfig_path() -> Option<String> {
    Some("power".into())
}
