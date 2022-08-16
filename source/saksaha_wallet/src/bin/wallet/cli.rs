use clap::{command, Arg, Command};
use saksaha_wallet::WalletError;

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) rpc_port: Option<u16>,
    pub(crate) public_key: Option<String>,
    pub(crate) secret: Option<String>,
    pub(crate) cfg_profile: Option<String>,
}

pub(crate) fn get_args() -> Result<CLIArgs, WalletError> {
    let app = create_app();

    let matches = app.get_matches();

    let rpc_port = match matches.value_of("rpc-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse rpc port (u16), err: {}",
                    err,
                )
                .into());
            }
        },
        None => None,
    };

    let public_key = match matches.value_of("public-key") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let secret = match matches.value_of("secret") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let cfg_profile = match matches.value_of("cfg-profile") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    Ok(CLIArgs {
        rpc_port,
        public_key,
        secret,
        cfg_profile,
    })
}

fn create_app<'a>() -> Command<'a> {
    command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sakaha network reference implementation")
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
            Arg::new("rpc-port") //
                .long("rpc-port")
                .takes_value(true)
                .long_help(
                    "Port to which bind RPC server \n\
                    e.g. 21452",
                ),
        )
        .arg(
            Arg::new("public-key") //
                .long("public-key")
                .takes_value(true)
                .long_help(
                    "Public Key \n\
                    e.g. 040c525ef532e1b9dfcbd2cb2566cdb8f2b0f324970e700d\
650d0190ded8c7b8519ef53afb9055d0c10d67d8fa55cf0f68ae153e356e3ca47545c2a00\
5154acdea",
                ),
        )
        .arg(
            Arg::new("secret") //
                .long("secret")
                .takes_value(true)
                .long_help(
                    "Secret (paired with public key) \n\
                    e.g. 4521ef8368476bf5bcc4e8784cb983ccaf42707abc2e2735\
8827a1f2f66d56d3",
                ),
        )
}
