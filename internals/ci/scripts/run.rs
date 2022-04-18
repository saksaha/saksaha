use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Run;

impl Script for Run {
    fn name() -> &'static str {
        "run"
    }

    fn define(app: Command) -> Command {
        app.subcommand(Command::new(Run::name()))
    }

    fn handle_matches(matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(Run::name()) {
            let program = "cargo";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args = [vec!["run", "--release", "-p", "saksaha", "--"], args]
                .concat();

            log!("Executing `{} {:?}`", program, args);

            Cmd::new(program)
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("failed to run");

            return Some(true);
        }

        None
    }
}
