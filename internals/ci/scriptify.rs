use clap::{ArgMatches, Command};

pub trait Scriptify {
    fn name(&self) -> &str;

    fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a>;

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool>;
}
