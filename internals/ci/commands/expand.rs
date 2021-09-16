use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{
    fs,
    io::{ErrorKind, Write},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

const NAME: &str = "expand";

pub fn expand_command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name(NAME)
            .setting(clap::AppSettings::AllowLeadingHyphen)
            .arg(Arg::with_name("args").multiple(true)),
    )
}

pub fn expand_exec(matches: &ArgMatches) {
    if let Some(_) = matches.subcommand_matches(NAME) {
        let is_rust_fmt = check_rustfmt();

        let dest = PathBuf::from_str(r"target/expand/debug")
            .expect("destination path");

        match std::fs::remove_dir_all(dest.to_owned()) {
            Err(err) if err.kind() == ErrorKind::NotFound => (),
            Err(err) => panic!("Cannot remove destination, err: {}", err),
            Ok(_) => {
                log!("Removed destination, {:?}\n", dest);
            }
        }

        fs::create_dir_all(dest.to_owned())
            .expect("Destination should be re-created");

        for p in ["bin", "lib"] {
            fs::create_dir_all(dest.join(p)).unwrap_or_else(|err| {
                panic!("{} should be created, err: {}", p, err)
            });
        }

        let bin_crates = vec!(vec!("saksaha", "sak"));
        let bin_path = dest.join("bin");
        expand(bin_crates, bin_path, is_rust_fmt);
        // expand_libs(dest, is_rust_fmt);
    }
}

fn check_rustfmt() -> bool {
    let is_rust_fmt = match Command::new("rustfmt").output() {
        Ok(_) => {
            log!("rustfmt is found, will format expanded output\n");
            true
        }
        Err(err) => {
            if let ErrorKind::NotFound = err.kind() {
                false
            } else {
                panic!("Error retrieving rustfmt, err: {}", err);
            }
        }
    };
    is_rust_fmt
}

fn execute_rustfmt(file_path: PathBuf) {
    log!("Executing rustfmt on file: {:?}\n", file_path);

    Command::new("rustfmt")
        .arg(file_path.to_str().expect("file path must be stringified"))
        .output()
        .unwrap_or_else(|err| {
            panic!("Rustfmt should be executed, err: {}", err);
        });
}

fn expand(crates: Vec<Vec<&str>>, dest: PathBuf, is_rust_fmt: bool) {
    for c in crates {
        let cmd = Command::new("cargo")
            .args(&[
                "rustc",
                "-p",
                c[0],
                "--bin",
                c[1],
                "--profile=check",
                "--",
                "-Zunpretty=expanded",
            ])
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| {
                panic!(
                    "Command should be executed, bin: {}, err: {}",
                    c[1], err
                )
            });

        let file_path = dest.join(format!("{}_{}.rs", c[0], c[1]));
        let mut f =
            fs::File::create(file_path.to_owned()).unwrap_or_else(|err| {
                panic!(
                    "Path should be created: {:?}, err: {}",
                    file_path.to_owned(),
                    err
                );
            });

        let output = cmd.wait_with_output().unwrap();

        f.write_all(&output.stdout)
            .expect("Expand output should be written");

        if is_rust_fmt {
            execute_rustfmt(file_path);
        }
    }
}

fn expand_libs(dest: PathBuf, is_rust_fmt: bool) {

}
