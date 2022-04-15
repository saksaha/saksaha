use super::{log, scripts};
use clap::{App, Arg, Command};

pub(super) fn run_app() -> Result<(), String> {
    let app = define_app();

    handle_cli_arg_matches(app);

    // let scripts = scripts::get_scripts();

    // for script in scripts.iter() {
    //     app = script.define(app.to_owned());
    // }

    // let matches = app.get_matches();

    // log!("Searching for script that can handle the request has been executed");

    // for script in scripts.iter() {
    //     if let Some(_) = script.handle_matches(&matches) {
    //         log!(
    //             "Finished running the script, exiting the process. \
    //             script: {}\n",
    //             script.name(),
    //         );

    //         std::process::exit(0);
    //     }
    // }

    return Err(format!(
        "Couldn't find any command to exeucte, Check the argument"
    ));
}

fn define_app() -> Command<'static> {
    let mut app = Command::new("CI")
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Rust saksaha implementation continuous integration toolsuite");

    log!("Registering subcommands");

    for (idx, script) in scripts::get_scripts().iter().enumerate() {
        log!("[{}] Subcommand: {}", idx, *script.0);

        app = app.subcommand(
            Command::new(*name)
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        );
    }

    app
}

fn handle_cli_arg_matches(app: Command) {
    let matches = app.get_matches();

    let name = "cmd";
    match name {
        "cmd" => {}
        _ => {}
    };
    // match matches.subcommand() {
    //     Some((cmd, arg_matches)) => {
    //         match cmd {
    //
    //         }
    //     }
    //     None => {}
    // };

    // if let Some(matches) = matches.subcommand_matches(self.name()) {}
}
