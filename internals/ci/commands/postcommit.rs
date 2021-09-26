use super::Commandify;
use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::{Command, Stdio};

pub struct Postcommit;

impl Commandify for Postcommit {
    fn name(&self) -> &str {
        "postcommit"
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
            let program = "git";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args = [vec!["log", "-1"], args].concat();

            log!("Executing `{} {:?}`\n", program, args,);

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
            println!("\n {}last commit message\n\n{}\n{}", opening, output, closing); 

            return Some(true);
        }
        None
    }
}
