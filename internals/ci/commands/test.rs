use super::Commandify;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Stdio};

pub struct Test;

impl Commandify for Test {
    fn name(&self) -> &str {
        "test"
    }

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::with_name(self.name())
                .allow_hyphen_values(true)
                .arg(Arg::with_name("args").multiple(true)),
        )
    }

    fn exec(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
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

            Command::new(program)
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
