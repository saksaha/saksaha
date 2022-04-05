use crate::scriptify::Scriptify;

mod build;
mod clean;
mod dev;
mod expand;
mod expand_release;
mod postcommit;
mod run;
mod test;

pub(crate) fn get_scripts() -> Vec<Box<dyn Scriptify + Send>> {
    let scripts: Vec<Box<dyn Scriptify + Send>> = vec![
        Box::new(build::Build),
        Box::new(dev::Dev),
        Box::new(run::Run),
        Box::new(clean::Clean),
        Box::new(expand::Expand),
        Box::new(expand_release::ExpandRelease),
        Box::new(postcommit::PostCommit),
        Box::new(test::Test),
    ];
    scripts
}
