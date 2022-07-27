use crate::log;
use colored::Colorize;
use std::process::Command;

pub(crate) struct Kommand;

impl Kommand {
    pub fn new(program: &str, args: Vec<String>) -> Command {
        log!(
            "Found subcommand, script: {}, executing `{} {}`",
            "dev",
            program.yellow(),
            args.join(" ").yellow(),
        );

        let mut cmd = Command::new(program);
        cmd.args(args);

        cmd
    }
}
