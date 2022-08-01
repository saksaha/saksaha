use crate::{log, CIError};
use colored::Colorize;
use std::{path::PathBuf, process::Command};

pub(crate) struct Kommand;

impl Kommand {
    pub fn new(
        program: &str,
        args: Vec<String>,
        curr_dir: Option<PathBuf>,
    ) -> Result<Command, CIError> {
        let curr_dir = curr_dir.unwrap_or(std::env::current_dir()?);

        log!(
            "Found subcommand, script: {}, curr_dir: {:?}, executing `{} {}`",
            "dev",
            curr_dir,
            program.yellow(),
            args.join(" ").yellow(),
        );

        let mut cmd = Command::new(program);
        cmd.current_dir(curr_dir);
        cmd.args(args);

        Ok(cmd)
    }
}
