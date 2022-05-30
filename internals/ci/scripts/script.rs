use super::BoxedError;
use clap::ArgMatches;

pub(crate) trait Script {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError>;
}
