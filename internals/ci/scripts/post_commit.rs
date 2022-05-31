use crate::scripts::BoxedError;
use crate::{log, script::Script};
use clap::{ArgMatches, Command};
use std::process::{Command as Cmd, Stdio};

pub(crate) struct PostCommit;

impl Script for PostCommit {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let program = "git";

        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };

        let args = [vec!["log", "-1"], args].concat();

        log!("Executing `{} {:?}`", program, args,);

        let cmd = Cmd::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to run");

        let output = cmd.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(&output.stdout);

        let opening = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";
        let closing = "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<";
        println!(
            "\n {}last commit message\n\n{}\n{}",
            opening, output, closing
        );

        Ok(())
    }
}
