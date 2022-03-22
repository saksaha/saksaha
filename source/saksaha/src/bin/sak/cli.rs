use clap::{arg, command, Command};

const DEFAULT_BOOTSTRAP_URLS: &str =
    include_str!("../../../../../config/bootstrap_urls");

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) config: Option<String>,
    pub(crate) rpc_port: Option<u16>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
}

pub(crate) fn get_args() -> Result<CLIArgs, String> {
    let matches = command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sakaha network reference implementation")
        .arg(arg!(-c --config [File]
                    "Saksaha configuration file, usually created at\n\
                    [[OS default config path]]/saksaha/config.json "))
        .arg(arg!(--"rpc-port" [Port] "Your RPC port"))
        .arg(arg!(--"disc-port" [Port] "Your P2P discovery port"))
        .arg(arg!(--"p2p-port" [Port] "Your p2p port"))
        .arg(arg!(--"bootstrap-urls" [Endpoints]
                "Bootstrap peer URLs to start discover, delimited by a comma,\n
                    e.g.\n
                        full url: sak://04715796a40b0d58fc14a3c4ebee21cb806763066a7f1a17adbc256999764443beb8109cfd000718535c5aa27513a2edafc6e8bdbe7c27edc2980f9bbc25142fc5@127.0.0.1:8080, \n
                        short url: 127.0.0.1:3030
                "))
        .get_matches();

    let config = match matches.value_of("config") {
        Some(c) => Some(String::from(c)),
        None => None,
    };

    let rpc_port = match matches.value_of("rpc-port") {
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

    let disc_port = match matches.value_of("disc-port") {
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

    let p2p_port = match matches.value_of("p2p-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(p) => Some(p),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the p2p port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let bootstrap_urls = match matches.values_of("bootstrap-urls") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    Ok(CLIArgs {
        config,
        rpc_port,
        disc_port,
        p2p_port,
        bootstrap_urls,
    })
}
