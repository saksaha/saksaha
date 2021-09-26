use clap::{App, Arg};
use logger::log;
use saksaha::{common::SakResult, err_res, node::Node, pconfig::PConfig};

struct Args {
    config: Option<String>,
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
}

fn get_args() -> SakResult<Args> {
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
            Arg::new("bootstrap_peers")
                .long("bootstrap-peers")
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
        Some(p) => {
            if let Err(err) = p.parse::<usize>() {
                return err_res!("Error parsing the rpc port, err: {}", err);
            }
            p.parse::<usize>().unwrap()
        }
        None => 0,
    };

    let disc_port = match flags.value_of("disc_port") {
        Some(p) => {
            if let Err(err) = p.parse::<usize>() {
                return err_res!("ERror parsing the rpc port, err: {}", err);
            }
            p.parse::<usize>().unwrap()
        }
        None => 0,
    };

    let bootstrap_peers = match flags.values_of("bootstrap_peers") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    Ok(Args {
        config,
        rpc_port,
        disc_port,
        bootstrap_peers,
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

    let node = match Node::new(
        args.rpc_port,
        args.disc_port,
        args.bootstrap_peers,
        pconf.p2p.public_key,
        pconf.p2p.secret,
    ) {
        Ok(n) => n,
        Err(err) => {
            log!(DEBUG, "Error creating a node, err: {}\n", err);
            std::process::exit(1);
        }
    };

    // match node.start() {
    //     Ok(_) => (),
    //     Err(err) => {
    //         log!(DEBUG, "Error starting a node, err: {}", err);
    //         std::process::exit(1);
    //     }
    // }
}

fn make_pconfig(config_path: Option<String>) -> PConfig {
    let pconf = match PConfig::of(config_path) {
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
