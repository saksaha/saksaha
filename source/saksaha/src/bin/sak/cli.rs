use super::app;
use clap::{arg, command, ArgMatches, Command};

const DEFAULT_BOOTSTRAP_URLS: &str =
    include_str!("../../../../../config/bootstrap_urls");

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) config: Option<String>,
    pub(crate) rpc_port: Option<u16>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) dev_mode: Option<String>,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
}

pub(crate) fn get_args() -> Result<CLIArgs, String> {
    let app = app::create_app();
    let matches = app.get_matches();

    let config = match matches.value_of("config") {
        Some(c) => Some(String::from(c)),
        None => None,
    };

    let rpc_port = match matches.value_of("rpc-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse rpc port (u16), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let disc_port = match matches.value_of("disc-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the disc port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let p2p_port = match matches.value_of("p2p-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the p2p port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let dev_mode = match matches.value_of("dev-mode") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let bootstrap_urls = match matches.values_of("bootstrap-urls") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    let disc_dial_interval = match matches.value_of("disc-dial-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p discovery dial interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let p2p_dial_interval = match matches.value_of("p2p-dial-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p dial interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let disc_table_capacity = match matches.value_of("disc-table-capacity") {
        Some(c) => match c.parse::<u16>() {
            Ok(capacity) => Some(capacity),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p discovery table capacity. Has to be u16,\
                    err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    Ok(CLIArgs {
        disc_dial_interval,
        disc_table_capacity,
        p2p_dial_interval,
        config,
        rpc_port,
        disc_port,
        p2p_port,
        dev_mode,
        bootstrap_urls,
    })
}

#[test]
fn test_if_app_matches_dev_mode() {
    let args = vec!["", "--dev-mode", "dev-local"];

    let app = app::create_app();
    let matches = app.get_matches_from(args);

    assert_eq!(matches.value_of("dev-mode"), Some("dev-local"));
}

#[test]
fn test_empty_dev_mode() {
    let args = vec![""];

    let app = app::create_app();
    let matches = app.get_matches_from(args);

    assert_eq!(matches.value_of("dev-mode"), None);
}
