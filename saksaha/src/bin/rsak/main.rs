use clap::{App, Arg};
use logger::log;
use saksaha::{
    node::{Node},
    pconfig::{PConfig},
    p2p::{Host},
};

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
                )
        )
        .arg(
            Arg::new("bootstrap_peers")
                .long("bootstrap-peers")
                .value_name("ENDPOINT")
                .use_delimiter(true)
                .about(
                    "Bootstrap peers to start discovery for",
                ),
        )
        .arg(
            Arg::new("rpc_port")
                .long("rpc-port")
                .value_name("PORT")
                .about(
                    "RPC port number",
                )
        )
        .get_matches();

    let pconf = make_config(flags.value_of("config"));

    let hconf = Host::new_config(
        flags.value_of("rpc_port"),
        flags.values_of("bootstrap_peers"),
        pconf.p2p.public_key,
        pconf.p2p.secret,
    );

    if let Err(ref err) = hconf {
        log!(DEBUG, "power");
        // return;
    }

    let n = Node::new(
        hconf.unwrap(),
    );

    n.start();
}

fn make_config(config_path: Option<&str>) -> PConfig {
    let pconf = PConfig::of(config_path);

    if let Err(err) = pconf {
        log!(
            DEBUG,
            "Error creating a persisted configuration, err: {}\n",
            err
        );
        std::process::exit(1);
    }

    let pconf = pconf.unwrap();
    log!(DEBUG, "Successfully loaded config, {:?}\n", pconf);
    pconf
}
