use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

const NAME: &str = "expand";

pub fn expand_command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name(NAME)
            .setting(clap::AppSettings::AllowLeadingHyphen)
            .arg(Arg::with_name("args").multiple(true)),
    )
}

pub fn expand_exec(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches(NAME) {
        let program = "cargo";
        let args = match matches.values_of("args") {
            Some(a) => a.collect(),
            None => vec![],
        };

        let dest = PathBuf::from_str(r"target/expand/debug")
            .expect("destination path");

        match std::fs::remove_dir_all(dest.to_owned()) {
            Err(err) if err.kind() == ErrorKind::NotFound => (),
            Err(err) => panic!("Cannot remove destination, err: {}", err),
            Ok(_) => (),
        }

        std::fs::create_dir_all(dest)
            .expect("Destination should be re-created");

        vec!("bin", "lib").iter().map(|p| {
            // std::fs::create_dir_all(dest.to_owned().join("bin").as_path())
            //     .expect("Destination should be re-created");
        });

        // let expand = std::path::PathBuf

        // let args = [vec!["clean", "--"], args].concat();
        // log!(
        //     "Executing `{} {:?}`\n",
        //     program,
        //     args,
        // );

        // Command::new(program)
        //     .args(args)
        //     .stdout(Stdio::inherit())
        //     .stderr(Stdio::inherit())
        //     .output()
        //     .expect("failed to run");
    }
}
