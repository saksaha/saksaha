use clap::{App, Arg};

fn main() {
    let matches = App::new("MyApp")
        .version("0.1")
        .author("Saksaha <team@saksaha.com>")
        .about("Saksaha node rust client")
        .license("MIT OR Apache-2.0")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    println!("{}", 123)

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // // let args = Cli {
    // //     pattern: pattern,
    // //     path: std::path::PathBuf::from(path),
    // // };

    // println!("Hello, world! {} {} ", pattern, path);
}
