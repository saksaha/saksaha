use clap::{command, Arg, Command};
use envelope_term::EnvelopeError;

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) cfg_profile: Option<String>,
    pub(crate) saksaha_endpoint: Option<String>,
}

pub(crate) fn get_args() -> Result<CLIArgs, EnvelopeError> {
    let app = create_app();

    let matches = app.get_matches();

    let cfg_profile = match matches.value_of("cfg-profile") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let saksaha_endpoint = match matches.value_of("saksaha-endpoint") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    Ok(CLIArgs {
        cfg_profile,
        saksaha_endpoint,
    })
}

fn create_app<'a>() -> Command<'a> {
    command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Envelope client application - Terminal UI")
        .allow_hyphen_values(true)
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
        .arg(
            Arg::new("saksaha-endpoint") //
                .long("saksaha-endpoint")
                .takes_value(true)
                .long_help(
                    "Endpoint to which bind saksaha node",
                ),
        )
}
