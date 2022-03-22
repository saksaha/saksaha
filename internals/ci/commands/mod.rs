mod build;
mod clean;
mod dev;
mod expand;
mod expand_release;
mod postcommit;
mod run;
mod test;

use clap::{ArgMatches, Command};

pub(crate) fn get_commands() -> Vec<Box<dyn Commandify + Send>> {
    let v: Vec<Box<dyn Commandify + Send>> = vec![
        Box::new(build::Build),
        Box::new(dev::Dev),
        Box::new(run::Run),
        Box::new(clean::Clean),
        Box::new(expand::Expand),
        Box::new(expand_release::ExpandRelease),
        Box::new(postcommit::Postcommit),
        Box::new(test::Test),
    ];
    v
}

pub trait Commandify {
    fn name(&self) -> &str;

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a>;

    fn exec(&self, matches: &ArgMatches) -> Option<bool>;
}
