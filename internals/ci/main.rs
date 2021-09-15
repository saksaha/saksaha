mod commands;
mod macros;

use clap::{App, Arg, SubCommand};
use commands::{
    COMMANDS
};

fn main() {
    let curr_dir = match std::env::current_dir() {
        Ok(cd) => cd
            .into_os_string()
            .into_string()
            .expect("current dir must be stringified"),
        Err(_) => {
            log!("Cannot get current working directory\n");

            std::process::exit(1);
        }
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
        log!(
            "Warning! You may be running this script not from the project \
            root with `ci` script"
        );
    }

    let mut app = App::new("CI")
        .version("0.1")
        .author("Saksaha <team@saksaha.com>")
        .about("Rust saksaha continuous integration toolsuite");

    let comm = COMMANDS
        .lock()
        .expect("cli command definitions must be loaded");

    for e in comm.iter() {
        app = (&e.def)(app);
    }

    let matches = app.get_matches();

    for e in comm.iter() {
        (e.exec)(&matches);
    }
}
