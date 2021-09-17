use super::{
    Commandify,
    expand,
};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{
    path::PathBuf,
    str::FromStr,
};

pub struct ExpandRelease;

impl Commandify for ExpandRelease {
    fn name(&self) -> &str {
        "expand_release"
    }

    fn def<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b> {
        app.subcommand(
            SubCommand::with_name(self.name())
                .setting(clap::AppSettings::AllowLeadingHyphen)
                .arg(Arg::with_name("args").multiple(true)),
        )
    }

    fn exec(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(_) = matches.subcommand_matches(self.name()) {
            let dest = PathBuf::from_str(r"target/expand/release")
                .expect("destination path");

            expand::expand(dest);

            return Some(true);
        }
        None
    }
}
