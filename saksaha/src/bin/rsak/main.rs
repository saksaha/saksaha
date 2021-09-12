use clap::{App, Arg};
use logger::log;
use saksaha::{
    node::{Node},
    pconfig::{PConfig},
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
                .takes_value(true),
        )
        .get_matches();

    let pconf = make_config(flags.value_of("config"));

    let n = Node::new();
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
