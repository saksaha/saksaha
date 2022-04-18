use super::Script;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::Command as Cmd;

pub(crate) struct Build;

impl Script for Build {
    fn name() -> &'static str {
        "build"
    }

    fn define(app: Command) -> Command {
        app.subcommand(
            Command::new(Build::name())
                .arg(Arg::new("args").multiple_occurrences(true)),
        )
    }

    fn handle_matches(matches: &ArgMatches) -> Option<bool> {
        // if let Some(matches) = matches.subcommand_matches(self.name()) {
        //     let program = "cargo";

        //     let args = match matches.values_of("args") {
        //         Some(a) => a.collect(),
        //         None => vec![],
        //     };

        //     let args = [vec!["build"], args].concat();

        //     log!("Executing `{} {:?}`", program, args,);

        //     let cmd =
        //         Cmd::new(program).args(args).spawn().expect("failed to run");

        //     cmd.wait_with_output().unwrap();

        //     return Some(true);
        // }

        None
    }
}

// pub(super) fn build(matches: &ArgMatches) {
//     let program = "cargo";

//     let args = match matches.values_of("args") {
//         Some(a) => a.collect(),
//         None => vec![],
//     };

//     let args = [vec!["build"], args].concat();

//     log!("Executing `{} {:?}`", program, args,);

//     let cmd = Cmd::new(program).args(args).spawn().expect("failed to run");

//     cmd.wait_with_output().unwrap();
// }
