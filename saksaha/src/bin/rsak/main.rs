use clap::{App, Arg};
use logger::log;
use saksaha::pconfig::{self, PConfig};

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

    let pconf = PConfig::of(flags.value_of("config"));

    // String::from("alkwj").chars();

    if let Err(err) = pconf {
        log!(
            DEBUG,
            "Error creating a persisted configuration, err: {}\n",
            err
        );
        std::process::exit(1);
    }

    // const c: &String = &format!("{}{}", a, b);
    // const c: &String = &format!("this parameter is for (default: {})", 1);
    // concat!(a, b);
    // const c: &str = b.to_string();

    // const c: String = format!("{}{}", a, b);
    // print!("44 {}\n", c);

    let pconf = pconf.unwrap();
    println!("Obtained configuration: {:?}", pconf);

    // let a = pconf.p2p.private_key.unwrap_or(String::from("power"));
    // println!("33 {}", a);

    // if let Some(c) = matches.value_of("config") {
    //     println!("Value for config: {}", c);
    // }

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // // let args = Cli {
    // //     pattern: pattern,
    // //     path: std::path::PathBuf::from(path),
    // // };

    // println!("Hello, world! {} {} ", pattern, path);
}
