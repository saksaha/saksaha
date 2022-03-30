use super::expand;
use crate::scriptify::Scriptify;
use clap::{Arg, ArgMatches, Command};
use std::{path::PathBuf, str::FromStr};

pub struct ExpandRelease;

impl Scriptify for ExpandRelease {
    fn name(&self) -> &str {
        "expand_release"
    }

    fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(Command::new(self.name()))
    }

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(_) = matches.subcommand_matches(self.name()) {
            let dest = PathBuf::from_str(r"target/expand/release")
                .expect("destination path");

            expand::expand(dest);

            return Some(true);
        }

        None
    }
}
