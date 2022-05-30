use super::Script;
use crate::log;
use crate::scripts::BoxedError;
use clap::{ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Run;

impl Script for Run {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let program = "cargo";
        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };
        let args =
            [vec!["run", "--release", "-p", "saksaha", "--"], args].concat();

        log!("Executing `{} {:?}`", program, args);

        Cmd::new(program)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");

        Ok(())
    }
}
