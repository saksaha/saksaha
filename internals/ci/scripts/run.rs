use crate::{log, scriptify::Scriptify};
use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub struct Run;

impl Scriptify for Run {
    fn name(&self) -> &str {
        "run"
    }

    fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(Command::new(self.name()))
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
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
