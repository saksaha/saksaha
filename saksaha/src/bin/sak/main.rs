use std::sync::Arc;

use clap::{App, Arg};
use logger::log;
use saksaha::{
    node::{status::Status, Node},
    pconfig::PConfig,
    process::Process,
};

const DEFAULT_BOOTSTRAP_URLS: &str =
    include_str!("../../../../config/bootstrap_urls");

struct Args {
    config: Option<String>,
    rpc_port: Option<u16>,
    disc_port: Option<u16>,
    bootstrap_endpoints: Option<Vec<String>>,
}

fn get_args() -> Result<Args, String> {
    let flags = App::new("Saksaha rust")
        .version("0.1")
        .author("Saksaha <team@saksaha.com>")
        .about("Saksaha node rust client")
        .license("MIT OR Apache-2.0")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about(
                    "Saksaha configuration file, usually created at \
                    [[OS default config path]]/saksaha/config.json",
                ),
        )
        .arg(
            Arg::new("bootstrap_endpoints")
                .long("bootstrap-endpoints")
                .value_name("ENDPOINT")
                .use_delimiter(true)
                .about("Bootstrap peers to start discovery for"),
        )
        .arg(
            Arg::new("rpc_port")
                .long("rpc-port")
                .value_name("PORT")
                .about("RPC port"),
        )
        .arg(
            Arg::new("disc_port")
                .long("disc-port")
                .value_name("PORT")
                .about("Discovery port"),
        )
        .get_matches();

    let config = match flags.value_of("config") {
        Some(c) => Some(String::from(c)),
        None => None,
    };

    let rpc_port = match flags.value_of("rpc_port") {
        Some(p) => match p.parse::<u16>() {
            Ok(p) => Some(p),
            Err(err) => {
                return Err(format!(
                    "Cannot parse rpc port (u16), err: {}",
                    err
                ));
            }
        },
        None => None,
    };

    let disc_port = match flags.value_of("disc_port") {
        Some(p) => match p.parse::<u16>() {
            Ok(p) => Some(p),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the disc port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let bootstrap_endpoints = match flags.values_of("bootstrap_endpoints") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    Ok(Args {
        config,
        rpc_port,
        disc_port,
        bootstrap_endpoints,
    })
}

fn main() {
    let args = match get_args() {
        Ok(a) => a,
        Err(err) => {
            log!(DEBUG, "Error parsing command line arguments, err: {}", err);

            std::process::exit(1);
        }
    };

    let pconf = make_pconfig(args.config);
    let node = Arc::new(Node::new());

    Process::init(node.clone());

    match node.start(
        args.rpc_port,
        args.disc_port,
        args.bootstrap_endpoints,
        pconf,
        DEFAULT_BOOTSTRAP_URLS,
    ) {
        Ok(_) => (),
        Err(err) => {
            log!(DEBUG, "Error starting a node, err: {}\n", err);
            std::process::exit(1);
        }
    };
}

fn make_pconfig(config_path: Option<String>) -> PConfig {
    let pconf = match PConfig::from_path(config_path) {
        Ok(p) => p,
        Err(err) => {
            log!(
                DEBUG,
                "Error creating a persisted configuration, err: {}\n",
                err
            );

            std::process::exit(1);
        }
    };

    log!(DEBUG, "Successfully loaded config, {:?}\n", pconf);
    pconf
}
