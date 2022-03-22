use super::Commandify;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Stdio};

pub struct Dev;

impl Commandify for Dev {
    fn name(&self) -> &str {
        "dev"
    }

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::new(self.name())
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

            let args = [vec!["run", "-p", "saksaha", "--"], args].concat();

            log!("Executing `{} {:?}`", program, args);

            std::env::set_var("RUST_BACKTRACE", "1");

            if std::env::var("LOG_LEVEL").is_err() {
                log!("LOG_LEVEL env var is not given, setting it to debug");
                std::env::set_var("LOG_LEVEL", "debug");
            }

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
