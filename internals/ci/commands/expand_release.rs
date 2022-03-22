use super::{expand, Commandify};
use clap::{Arg, ArgMatches, Command};
use std::{path::PathBuf, str::FromStr};

pub struct ExpandRelease;

impl Commandify for ExpandRelease {
    fn name(&self) -> &str {
        "expand_release"
    }

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::with_name(self.name())
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
