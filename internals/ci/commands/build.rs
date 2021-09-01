use super::Commandify;
use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::{Command};

pub struct Build;

impl Commandify for Build {
    fn name(&self) -> &str {
        "build"
    }

    fn def<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        app.subcommand(
            SubCommand::with_name(self.name())
                .setting(clap::AppSettings::AllowLeadingHyphen)
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
            let args = [vec!["build"], args].concat();

            log!(
                "Executing `{} {:?}`\n",
                program,
                args,
            );

            let cmd = Command::new(program)
                .args(args)
                .spawn()
                .expect("failed to run");

            cmd.wait_with_output().unwrap();

            return Some(true);
        }
        None
    }
}
