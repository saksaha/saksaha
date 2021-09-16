use super::{
    Commandify,
    expand,
};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{
    path::PathBuf,
    str::FromStr,
};

const NAME: &str = "expand_release";

pub struct ExpandRelease;

impl Commandify for ExpandRelease {
    fn def<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        app.subcommand(
            SubCommand::with_name(NAME)
                .setting(clap::AppSettings::AllowLeadingHyphen)
                .arg(Arg::with_name("args").multiple(true)),
        )
    }

    fn exec(&self, matches: &ArgMatches) {
        if let Some(_) = matches.subcommand_matches(NAME) {
            let dest = PathBuf::from_str(r"target/expand/release")
                .expect("destination path");

            expand::expand(dest);
        }
    }
}
