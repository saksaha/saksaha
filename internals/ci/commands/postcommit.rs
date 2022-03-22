use super::Commandify;
use crate::log;
use clap::{Arg, ArgMatches, Command};
use std::process::{Stdio};

pub struct Postcommit;

impl Commandify for Postcommit {
    fn name(&self) -> &str {
        "postcommit"
    }

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::new(self.name())
                .arg(Arg::new("args").multiple(true))
        )
    }

    fn exec(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
            let program = "git";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args = [vec!["log", "-1"], args].concat();

            log!("Executing `{} {:?}`", program, args,);

            let cmd = Command::new(program)
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

            return Some(true);
        }
        None
    }
}
