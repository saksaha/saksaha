mod macros;
mod commands;

use clap::{App, Arg, SubCommand};


fn main() {
    let curr_dir = match std::env::current_dir() {
        Ok(cd) => {
            cd.into_os_string().into_string()
                .expect("current dir must be stringified")
        },
        Err(_) => {
            log!("Cannot get current working directory\n");

            std::process::exit(1);
        },
    };

    let project_root = match std::env::var("PROJECT_ROOT") {
        Ok(p) => p,
        Err(_) => {
            log!(
                "PROJECT_ROOT is not defined. This is most likely due to that \
                this binary has not been executed from the project root. Use `ci` \
                script in the project. Exiting.\n"
            );

            std::process::exit(1);
        }
    };

    log!(
        "CI is starting, project root: {}, current workign directory: {}\n",
        project_root,
        curr_dir,
    );

    if project_root != curr_dir {
        log!("Warning! You may be running this script not from the project \
            root with `ci` script");
    }

    commands::t();

    let matches = App::new("CI")
        .version("0.1")
        .author("Saksaha <team@saksaha.com>")
        .about("Rust saksaha continuous integration toolsuite")
        .subcommand(
            SubCommand::with_name("dev")
                .version("0.1")
                .arg(
                    Arg::with_name("args")
                        .multiple(true)
                ),
        )
        .get_matches();

        if let Some(o) = matches.value_of("output") {
            println!("Value for output: {}", o);
        }

        if let Some(o) = matches.value_of("subcommand") {
            println!("fff {}", o);
        }

        if let Some(matches) = matches.subcommand_matches("dev") {
            if let Some(args) = matches.values_of("args") {
                let args: Vec<_> = args.collect();

            }

            // print!("{:?}\n", b);
            // "$ myapp test" was run
            // if matches.is_present("list") {
            //     // "$ myapp test -l" was run
            //     println!("Printing testing lists...");
            // } else {
            //     println!("Not printing testing lists...");
            // }
        }
}
