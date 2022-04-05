use crate::{log, scriptify::Scriptify};
use clap::{arg, Arg, ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub struct Dev;

impl Scriptify for Dev {
    fn name(&self) -> &str {
        "dev"
    }

    fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        let app = app.subcommand(
            Command::new(self.name())
                .arg(Arg::new("SAKSAHA_ARGS").multiple_values(true))
                .allow_hyphen_values(true),
        );

        app
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
            let program = "cargo";

            let args = match matches.values_of("SAKSAHA_ARGS") {
                Some(a) => a.collect(),
                None => vec![],
            };

            let args =
                [vec!["run", "--package", "saksaha", "--"], args].concat();

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
        None
    }
}
