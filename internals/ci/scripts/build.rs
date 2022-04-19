use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::Command as Cmd;

pub(crate) struct Build;

impl Script for Build {
    fn name(&self) -> &'static str {
        "build"
    }

    fn define<'a>(&'a self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::new("build")
                .arg(Arg::new("args").multiple_occurrences(true)),
        )
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        let program = "cargo";

        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };

        let args = [vec!["build"], args].concat();

        log!("Executing `{} {:?}`", program, args,);

        let cmd = Cmd::new(program).args(args).spawn().expect("failed to run");

        cmd.wait_with_output().unwrap();

        return Some(true);
    }
}
