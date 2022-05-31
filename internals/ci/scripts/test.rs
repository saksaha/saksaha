use crate::scripts::BoxedError;
use crate::{log, script::Script};
use clap::ArgMatches;
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Test;

impl Script for Test {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
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

        Ok(())
    }
}
