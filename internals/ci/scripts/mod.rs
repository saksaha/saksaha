use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) mod build;
pub(crate) mod clean;
pub(crate) mod dev;
pub(crate) mod expand;
pub(crate) mod post_commit;
pub(crate) mod run;
pub(crate) mod test;

use build::Build;
use clean::Clean;
use dev::Dev;
use expand::Expand;
use post_commit::PostCommit;
use run::Run;
use test::Test;

// crate::load_script![Build, Clean, Dev, Expand, PostCommit, Run, Test];

// pub(crate) fn get_scripts() -> Vec<Box<dyn Script + Send>> {
//     let scripts: Vec<Box<dyn Script + Send>> = vec![
//         Box::new(build::Build),
//         Box::new(dev::Dev),
//         Box::new(run::Run),
//         Box::new(clean::Clean),
//         Box::new(expand::Expand),
//         Box::new(post_commit::PostCommit),
//         Box::new(test::Test),
//     ];
//     scripts
// }

// pub(crate) trait Scriptable {
//     fn name() -> &'static str;

//     fn define(app: Command<'static>) -> Command<'static>;

//     fn handle_matches(matches: &ArgMatches) -> Option<bool>;
// }

// pub(crate) enum Script {
//     Build(Build),
//     Clean(clean::Clean),
//     Dev(dev::Dev),
//     Expand(expand::Expand),
//     PostCommit(post_commit::PostCommit),
//     Run(run::Run),
//     Test(test::Test),
// }

pub(crate) trait Script {
    fn name() -> &'static str;

    fn define(app: Command<'static>) -> Command<'static>;

    fn handle_matches(matches: &ArgMatches) -> Option<bool>;
}

#[macro_export]
macro_rules! load_script {
    ($($script:ident),+) => {
        // pub(crate) enum Script {
        //     $($script($script),)+
        // }

        // lazy_static! {
        //     pub(crate) static ref SCRIPTS: HashMap<&'static str, Box<dyn Script + Send + Sync>> = {
        //         let mut m = HashMap::new();
        //         // let b = Box::new(build::Build);

        //         // $(
        //         //     // let b = Box::new(Build);
        //         //     // m.insert($script::name(), b);
        //         // )*

        //         m
        //     };
        // }
    };
}
