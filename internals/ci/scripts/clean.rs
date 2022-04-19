use super::Script;
use crate::log;
use clap::{ArgMatches, Command};
use std::process::Command as Cmd;

pub(crate) struct Clean;

impl Script for Clean {
    fn name(&self) -> &'static str {
        "clean"
    }

    fn define<'a>(&'a self, app: Command<'a>) -> Command<'a> {
        app.subcommand(Command::new(self.name()))
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        let program = "cargo";

        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };
        let args = [vec!["clean", "--"], args].concat();

        log!("Executing `{} {:?}`", program, args,);

        let cmd = Cmd::new(program).args(args).spawn().expect("failed to run");

        cmd.wait_with_output().unwrap();

        return Some(true);
    }
}
