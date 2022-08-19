use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{Arg, ArgMatches};
use envelope_term::term::TermArgs;
use envelope_term::EnvelopeError;
use envelope_term::{pconfig::PConfig, term};
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
        .arg(
            Arg::new("cfg-profile") //
                .long("cfg-profile")
                .takes_value(true)
                .long_help(
                    "Config profile. This dictates which 'config (credential)' \
                    to load, \n
                    e.g. 'dev_local_1'",
                ),
        )
        .subcommand(Command::new("test").about("does testing things").arg(
            arg!(-l --list "lists test values").action(ArgAction::SetTrue),
        ))
        .get_matches();

    matches
}

fn main() -> Result<(), EnvelopeError> {
    let cli_args = get_cli_args();

    let term_args = resolve_pconfig_path(cli_args);

    let config = PConfig::new(&term_args.cfg_profile)?;

    term::run(term_args)?;

    Ok(())
}

fn resolve_pconfig_path(matches: ArgMatches) -> TermArgs {
    let cfg_profile = match matches.value_of("cfg-profile") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    TermArgs { cfg_profile }
}
