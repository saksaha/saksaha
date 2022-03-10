mod build;
mod clean;
mod dev;
mod expand;
mod expand_release;
mod postcommit;
mod run;
mod test;

use clap::{App, ArgMatches};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static COMMANDS: Lazy<Mutex<Vec<Box<dyn Commandify + Send>>>> =
    Lazy::new(|| {
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
        Mutex::new(v)
    });

pub trait Commandify {
    fn name(&self) -> &str;

    fn def<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>;

    fn exec(&self, matches: &ArgMatches) -> Option<bool>;
}
