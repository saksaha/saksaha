use clap::{arg, command, Command};
use logger::{terr, tinfo};
use saksaha::{pconfig::PConfig, system::System};

const DEFAULT_BOOTSTRAP_URLS: &str =
    include_str!("../../../../../config/bootstrap_urls");

#[derive(Debug)]
struct Args {
    config: Option<String>,
    rpc_port: Option<u16>,
    disc_port: Option<u16>,
    p2p_port: Option<u16>,
    bootstrap_endpoints: Option<Vec<String>>,
}

fn get_args() -> Result<Args, String> {
    let matches = command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sakaha network reference implementation")
        .arg(arg!(-c --config [File]
                    "Saksaha configuration file, usually created at\n\
                    [[OS default config path]]/saksaha/config.json "))
        .arg(arg!(--rpc_port [Port] "Your RPC port"))
        .arg(arg!(--disc_port [Port] "Your P2P discovery port"))
        .arg(arg!(--p2p_port [Port] "Your p2p port"))
        .arg(arg!(--bootstrap_endpoints [Endpoints]
                "Bootstrap peer URLs to start discover, delimited by a comma"))
        .get_matches();

    // let flags = Command::new("Saksaha rust")
    //     .version("0.1")
    //     .author("Saksaha <team@saksaha.com>")
    //     .about("Saksaha network rust client")
    //     .arg(
    //         Arg::new("config")
    //             .short('c')
    //             .long("config")
    //             .value_name("FILE")
    //             .about(
    //                 "Saksaha configuration file, usually created at \
    //                 [[OS default config path]]/saksaha/config.json",
    //             ),
    //     )
    //     .arg(
    //         Arg::new("bootstrap_urls")
    //             .long("bootstrap-urls")
    //             .value_name("ENDPOINT")
    //             .use_delimiter(true)
    //             .about("Bootstrap peers to start discovery for"),
    //     )
    //     .arg(
    //         Arg::new("rpc_port")
    //             .long("rpc-port")
    //             .value_name("PORT")
    //             .about("RPC port"),
    //     )
    //     .arg(
    //         Arg::new("disc_port")
    //             .long("disc-port")
    //             .value_name("PORT")
    //             .about("Discovery port"),
    //     )
    //     .arg(
    //         Arg::new("p2p_port")
    //             .long("p2p-port")
    //             .value_name("PORT")
    //             .about("P2P port"),
    //     )
    //     .get_matches();

    let config = match matches.value_of("config") {
        Some(c) => Some(String::from(c)),
        None => None,
    };

    let rpc_port = match matches.value_of("rpc_port") {
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

    let disc_port = match matches.value_of("disc_port") {
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

    let p2p_port = match matches.value_of("p2p_port") {
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

    let bootstrap_endpoints = match matches.values_of("bootstrap_endpoints") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    Ok(Args {
        config,
        rpc_port,
        disc_port,
        p2p_port,
        bootstrap_endpoints,
    })
}

fn main() {
    print!("Saksaha is launching...\n");

    logger::init();

    let args = match get_args() {
        Ok(a) => {
            tinfo!("saksaha", "Arguments parsed: {:?}", a);

            a
        }
        Err(err) => {
            terr!("Can't parse command line arguments, err: {}", err);

            std::process::exit(1);
        }
    };

    let pconf = {
        let c = match PConfig::from_path(args.config) {
            Ok(p) => p,
            Err(err) => {
                terr!(
                    "saksaha",
                    "Error creating a persisted configuration, err: {}",
                    err
                );

                std::process::exit(1);
            }
        };

        tinfo!("saksaha", "Persisted config loaded, conf: {:?}", c);

        c
    };

    let system = System::new();

    match system.start(
        args.rpc_port,
        args.disc_port,
        args.p2p_port,
        args.bootstrap_endpoints,
        pconf,
        DEFAULT_BOOTSTRAP_URLS,
    ) {
        Ok(_) => (),
        Err(err) => {
            terr!("saksaha", "Can't start the system, err: {}", err);

            std::process::exit(1);
        }
    };
}
