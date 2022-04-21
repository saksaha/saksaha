use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Dev;

impl Script for Dev {
    fn name(&self) -> &'static str {
        "dev"
    }

    fn define<'a>(&self, app: Command<'a>) -> Command<'a> {
        let app = app.subcommand(
            Command::new(self.name())
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        );

        app
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        let program = "cargo";

        let args = match matches.values_of("SAKSAHA_ARGS") {
            Some(a) => a.collect(),
            None => vec![],
        };

        let args = [vec!["run", "--package", "saksaha", "--"], args].concat();

        log!(
            "Found subcommand, script: {}, executing `{} {}`",
            self.name(),
            program,
            args.join(" "),
        );

        std::env::set_var("RUST_BACKTRACE", "1");

        if std::env::var("LOG_LEVEL").is_err() {
            log!("LOG_LEVEL env var is not given, setting it to debug");
            std::env::set_var("LOG_LEVEL", "debug");
        }

        Cmd::new(program)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");

        return Some(true);
    }
}