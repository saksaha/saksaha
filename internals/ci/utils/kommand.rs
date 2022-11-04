use crate::{log, logln, CIError};
use colored::Colorize;
use std::{ffi::OsStr, path::PathBuf, process::Command};

pub(crate) struct Kommand;

impl Kommand {
    pub fn new(
        program: &str,
        // OS string is 'owned' string, since different OSs have different specs.
        args: Vec<String>,
        curr_dir: Option<PathBuf>,
    ) -> Result<Command, CIError> {
        let curr_dir = curr_dir.unwrap_or(std::env::current_dir()?);

        let cmd_finalized = format!("{} {}", program, args.join(" "));

        logln!("Kommand curr_dir: {:?}", curr_dir,);

        logln!("Kommand executing '{}'", cmd_finalized,);

        let mut cmd = Command::new(program);
        cmd.current_dir(curr_dir);
        cmd.args(args);

        Ok(cmd)
    }
}
