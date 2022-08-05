use clap::{command, Arg, Command};
use saksaha_wallet::WalletError;

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) app_prefix: Option<String>,
    pub(crate) rpc_port: Option<u16>,
    pub(crate) id: Option<String>,
    pub(crate) key: Option<String>,
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

    let app_prefix = match matches.value_of("app-prefix") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let id = match matches.value_of("id") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let key = match matches.value_of("key") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    Ok(CLIArgs {
        rpc_port,
        app_prefix,
        id,
        key,
    })
}

fn create_app<'a>() -> Command<'a> {
    command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sakaha network reference implementation")
        .allow_hyphen_values(true)
        .arg(
            Arg::new("app-prefix") //
                .long("app-prefix")
                .takes_value(true)
                .long_help(
                    "Saksaha app prefix. This makes all the member paths
                    including db directories created under \n\
                    APP_PATH/{app_prefix}/{db_dirs}, as in \n\
                    APP_PATH/app_path/ledger.",
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
            Arg::new("id") //
                .long("id")
                .takes_value(true)
                .long_help(
                    "User identifier \n\
                    e.g. User_1234",
                ),
        )
        .arg(
            Arg::new("key") //
                .long("cipher key")
                .takes_value(true)
                .long_help(
                    "Key by which encrypt and decrypt data \n\
                    e.g. LKJ3jfkwala11fass",
                ),
        )
}
