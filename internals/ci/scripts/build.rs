use crate::log;
use crate::scriptify::Scriptify;
use clap::{Arg, ArgMatches, Command};
use std::process::Command as Cmd;

pub struct Build1;

impl Scriptify for Build {
    fn name(&self) -> &str {
        "build"
    }

    fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        let a = || {};
        app.subcommand(
            Command::new(self.name())
                .arg(Arg::new("args").multiple_occurrences(true)),
        )
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
            let program = "cargo";

            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };

            let args = [vec!["build"], args].concat();

            log!("Executing `{} {:?}`", program, args,);

            let cmd =
                Cmd::new(program).args(args).spawn().expect("failed to run");

            cmd.wait_with_output().unwrap();

            return Some(true);
        }

        None
    }
}
