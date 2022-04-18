use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Test;

impl Script for Test {
    fn name() -> &'static str {
        "test"
    }

    fn define(app: Command) -> Command {
        app.subcommand(Command::new(Test::name()))
    }

    fn handle_matches(matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(Test::name()) {
            let program = "cargo";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args =
                [vec!["test", "--", "--nocapture", "--show-output"], args]
                    .concat();

            // let args = [vec!["test", "--"], args]
            //     .concat();

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
