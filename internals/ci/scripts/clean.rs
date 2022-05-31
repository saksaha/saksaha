use crate::scripts::BoxedError;
use crate::{log, script::Script};
use clap::{ArgMatches, Command};
use std::process::Command as Cmd;

pub(crate) struct Clean;

impl Script for Clean {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let program = "cargo";

        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };
        let args = [vec!["clean", "--"], args].concat();

        log!("Executing `{} {:?}`", program, args,);

        let cmd = Cmd::new(program).args(args).spawn().expect("failed to run");

        cmd.wait_with_output().unwrap();

        Ok(())
    }
}
