use clap::{App, Arg};
use logger::log;
use saksaha::{node::Node, pconfig::PConfig};

fn main() {
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
                .about("RPC port number"),
        )
        .get_matches();

    let pconf = make_pconfig(flags.value_of("config"));

    let node = match Node::new(
        flags.value_of("rpc_port"),
        flags.values_of("bootstrap_peers"),
        pconf.p2p.public_key,
        pconf.p2p.secret,
    ) {
        Ok(n) => n,
        Err(err) => {
            log!(DEBUG, "Error creating a node, err: {}\n", err);
            std::process::exit(1);
        }
    };

    match node.start() {
        Ok(_) => (),
        Err(err) => {
            log!(DEBUG, "Error starting a node, err: {}", err);
            std::process::exit(1);
        }
    }
}

fn make_pconfig(config_path: Option<&str>) -> PConfig {
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
