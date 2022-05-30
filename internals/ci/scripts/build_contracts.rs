use super::Script;
use crate::log;
use crate::scripts::BoxedError;
use clap::ArgMatches;
use std::process::Command as Cmd;

pub(crate) struct BuildContracts;

impl Script for BuildContracts {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let args = ["build", "source/saksaha/src/contracts"];

        let cmd = Cmd::new("wasm-pack")
            .args(args)
            .spawn()
            .expect("failed to run");

        cmd.wait_with_output().unwrap();

        Ok(())
    }
}
