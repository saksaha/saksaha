mod macros;

use clap::{App, Arg, SubCommand};
use std::{arch::x86_64::_mm_load_ps1, path::PathBuf};

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

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("dev")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                // .arg(
                //     Arg::with_name("debug")
                //         .short("d")
                //         .help("print debug information verbosely"),
                // ),
        )
        .get_matches();

        if let Some(o) = matches.value_of("output") {
            println!("Value for output: {}", o);
        }

        if let Some(o) = matches.value_of("subcommand") {
            println!("fff {}", o);
        }

        if let Some(matches) = matches.subcommand_matches("dev") {
            log!("power");
            // "$ myapp test" was run
            // if matches.is_present("list") {
            //     // "$ myapp test -l" was run
            //     println!("Printing testing lists...");
            // } else {
            //     println!("Not printing testing lists...");
            // }
        }

}
