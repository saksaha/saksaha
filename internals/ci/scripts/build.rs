use crate::log;
use crate::script::Script;
use crate::scripts::BoxedError;
use clap::ArgMatches;
use std::process::Command as Cmd;

pub(crate) struct Build;

impl Script for Build {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let program = "cargo";

        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };

        let args = [vec!["build"], args].concat();

        log!("Executing `{} {:?}`", program, args,);

        let cmd = Cmd::new(program).args(args).spawn().expect("failed to run");

        cmd.wait_with_output().unwrap();

        Ok(())
    }
}
