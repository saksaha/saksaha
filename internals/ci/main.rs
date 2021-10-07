mod commands;
mod macros;

use clap::App;
use commands::COMMANDS;

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
                "PROJECT_ROOT is not defined. This is most likely due to \
                that this binary has not been executed from the project root. \
                Use `ci` script in the project. Exiting.\n"
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
        app = e.def(app.to_owned());
    }

    let matches = app.get_matches();

    for e in comm.iter() {
        if let Some(_) = e.exec(&matches) {
            log!("Command has been executed, name: {}\n", e.name());

            std::process::exit(0);
        }
    }

    log!("Couldn't find any command to exeucte, Check the argument\n");
}
