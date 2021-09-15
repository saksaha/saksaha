use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::{Command, Stdio};

const NAME: &str = "clean";

pub fn clean_command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name(NAME)
            .setting(clap::AppSettings::AllowLeadingHyphen)
            .arg(Arg::with_name("args").multiple(true)),
    )
}

pub fn clean_exec(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches(NAME) {
        let program = "cargo";
        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };
        let args = [vec!["clean", "--"], args].concat();

        log!(
            "Executing `{} {:?}`\n",
            program,
            args,
        );

        Command::new(program)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");
    }
}
