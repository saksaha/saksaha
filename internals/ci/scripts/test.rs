use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Test;

impl Script for Test {
    fn name(&self) -> &'static str {
        "test"
    }

    fn define<'a>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::new(self.name())
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        )
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        let program = "cargo";

        let args = match matches.values_of("SAKSAHA_ARGS") {
            Some(a) => a.collect::<Vec<&str>>().join(" "),
            None => String::new(),
        };

        let args =
            vec!["test", &args[..], "--", "--nocapture", "--show-output"];

        log!("Executing `{} {:?}`", program, args);

        Cmd::new(program)
            .env("RUST_LOG", "debug")
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");

        return Some(true);
    }
}
