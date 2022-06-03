use crate::script::Script;
use crate::scripts;
use crate::scripts::BoxedError;
use clap::Arg;
use clap::Command;

pub(super) fn run_app() -> Result<(), String> {
    let app = define_app();

    let _ = handle_cli_arg_matches(app);

    Ok(())
}

fn define_app() -> Command<'static> {
    let app = Command::new("CI")
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Rust saksaha implementation continuous integration toolsuite")
        //
        .subcommand(
            Command::new("build")
                .arg(Arg::new("args").multiple_occurrences(true)),
        )
        //
        .subcommand(
            Command::new("build_contracts")
                .arg(Arg::new("args").multiple_occurrences(true)),
        )
        //
        .subcommand(Command::new("clean"))
        //
        .subcommand(
            Command::new("dev")
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        )
        //
        .subcommand(Command::new("expand"))
        //
        .subcommand(Command::new("postcommit"))
        //
        .subcommand(Command::new("run"))
        //
        .subcommand(
            Command::new("test")
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        );

    app
}

fn handle_cli_arg_matches(app: Command) -> Result<(), BoxedError> {
    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("dev", m)) => scripts::Dev::handle_matches(m),
        Some(("build", m)) => scripts::Build::handle_matches(m),
        Some(("clean", m)) => scripts::Clean::handle_matches(m),
        Some(("build_contracts", m)) => {
            scripts::BuildContracts::handle_matches(m)
        }
        Some(("expand", m)) => scripts::Expand::handle_matches(m),
        Some(("post_commit", m)) => scripts::PostCommit::handle_matches(m),
        Some(("run", m)) => scripts::Run::handle_matches(m),
        Some(("test", m)) => scripts::Test::handle_matches(m),
        _ => Err(format!("Cannot find the script").into()),
    };

    result
}
