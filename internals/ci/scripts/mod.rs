use crate::scriptify::Scriptify;
use clap::{Arg, ArgMatches, Command};

mod build;
mod clean;
mod dev;
mod expand;
mod expand_release;
mod postcommit;
mod run;
mod test;

// pub(crate) fn get_scripts() -> Vec<Box<dyn Scriptify + Send>> {
//     let scripts: Vec<Box<dyn Scriptify + Send>> = vec![
//         Box::new(build::Build),
//         Box::new(dev::Dev),
//         Box::new(run::Run),
//         Box::new(clean::Clean),
//         Box::new(expand::Expand),
//         Box::new(expand_release::ExpandRelease),
//         Box::new(postcommit::PostCommit),
//         Box::new(test::Test),
//     ];
//     scripts
// }

type ScriptMainFunction = fn(matches: &ArgMatches);

type ScriptDef = (&'static str, Script);

const a: ScriptMainFunction = build::build;

pub(crate) enum Script {
    Build(&'static str, ScriptMainFunction),
    Dev(&'static str, ScriptMainFunction),
}

// impl Script {
//     fn value(&self) {
//         match *self {}
//     }
// }

pub(crate) fn get_scripts<'a>() -> Vec<Script> {
    vec![
        Script::Build("build", build::build),
        Script::Dev("dev", dev::dev),
    ]
}
