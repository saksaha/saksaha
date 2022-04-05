use clap::Command;

mod log;
mod scriptify;
mod scripts;

fn main() {
    let curr_dir = match std::env::current_dir() {
        Ok(cd) => cd
            .into_os_string()
            .into_string()
            .expect("current dir must be stringified"),
        Err(_) => {
            log!("Cannot get current working directory");

            std::process::exit(1);
        }
    };

    let project_root = match std::env::var("PROJECT_ROOT") {
        Ok(p) => p,
        Err(_) => {
            log!(
                "PROJECT_ROOT is not defined. This is most likely due to \
                that this binary has not been executed from the project root. \
                Use `ci` script in the project. Exiting."
            );

            std::process::exit(1);
        }
    };

    log!(
        "CI is starting, project root: {}, current working directory: {}",
        project_root,
        curr_dir,
    );

    if project_root != curr_dir {
        log!(
            "Warning! You may be running this script not from the project \
            root with `ci` script"
        );
    }

    let mut app = Command::new("CI")
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Rust saksaha implementation continuous integration toolsuite");

    let scripts = scripts::get_scripts();

    for script in scripts.iter() {
        app = script.define(app.to_owned());
    }

    let matches = app.get_matches();

    log!("Searching for script that can handle the request has been executed");

    for script in scripts.iter() {
        if let Some(_) = script.handle_matches(&matches) {
            log!(
                "Finished running the script, exiting the process. \
                script: {}\n",
                script.name(),
            );

            std::process::exit(0);
        }
    }

    log!("Couldn't find any command to exeucte, Check the argument");
}
