use crate::utils::Kommand;
use crate::CIError;
use colored::Colorize;
use std::env::Args;
use std::process::{Command as Cmd, Stdio};

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "cargo";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let args_1: Vec<String> = ["run", "--package", "saksaha_wallet", "--"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let args = [args_1, cli_args].concat();

    use std::io::{BufRead, BufReader, BufWriter, Write};
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

    let proc = Kommand::new(program, args, None)?
        .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        // .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to run");

    // let _ = cmd.wait_with_output();

    proc.stdin.as_ref().unwrap().write(b"alwkej").unwrap();

    let _ = proc.wait_with_output();

    Ok(())
}
