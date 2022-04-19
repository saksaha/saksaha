use clap::{ArgMatches, Command};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) mod build;
pub(crate) mod clean;
pub(crate) mod dev;
pub(crate) mod expand;
pub(crate) mod post_commit;
pub(crate) mod run;
pub(crate) mod test;

crate::load_script![
    build::Build,
    clean::Clean,
    dev::Dev,
    expand::Expand,
    post_commit::PostCommit,
    run::Run,
    test::Test,
];

pub(crate) trait Script {
    fn name(&self) -> &'static str;

    fn define<'a>(&'a self, app: Command<'a>) -> Command<'a>;

    fn handle_matches(&self, matches: &ArgMatches) -> Option<bool>;
}

#[macro_export]
macro_rules! load_script {
    ($($x:expr),+ $(,)?) => {
        lazy_static! {
            pub(crate) static ref SCRIPTS:
                HashMap<&'static str, Box<dyn Script + Sync + Send>> = {

                let mut m = HashMap::new();

                $(
                    let scr = Box::new($x) as Box<dyn Script + Sync + Send>;
                    m.insert(scr.name(), scr);
                )+

                m
            };
        }
    };
}
