use super::log;
use super::scripts;
use clap::Command;

pub(super) fn run_app() -> Result<(), String> {
    let app = define_app();

    let _ = handle_cli_arg_matches(app);

    Ok(())
}

fn define_app() -> Command<'static> {
    let mut app = Command::new("CI")
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Rust saksaha implementation continuous integration toolsuite");

    log!("Registering subcommands");

    for (name, script) in scripts::SCRIPTS.iter() {
        log!("  - script name: {}", name);

        app = script.define(app);
    }

    app
}

fn handle_cli_arg_matches(app: Command) -> Result<(), String> {
    let matches = app.get_matches();

    match matches.subcommand() {
        Some((subcmd, arg_matches)) => {
            match scripts::SCRIPTS.get(subcmd) {
                Some(s) => {
                    log!("Found subcmd, running: {}", subcmd);

                    s.handle_matches(arg_matches);
                }
                None => {
                    return Err(format!(
                        "Subcommand is not registered, subcmd: {}",
                        subcmd
                    ))
                }
            };
        }
        None => {
            println!("No match");
        }
    };

    Ok(())
}
