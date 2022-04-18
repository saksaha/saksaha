use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::Command as Cmd;

pub(crate) struct Clean;

impl Script for Clean {
    fn name() -> &'static str {
        "clean"
    }

    fn define(app: Command) -> Command {
        app.subcommand(Command::new(Clean::name()))
    }

    fn handle_matches(matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(Clean::name()) {
            let program = "cargo";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args = [vec!["clean", "--"], args].concat();

            log!("Executing `{} {:?}`", program, args,);

            let cmd =
                Cmd::new(program).args(args).spawn().expect("failed to run");

            cmd.wait_with_output().unwrap();

            return Some(true);
        }
        None
    }
}
