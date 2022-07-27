use crate::{log, CIError};
use std::{
    fs,
    io::{ErrorKind, Write},
    path::PathBuf,
    process::{Command as Cmd, Stdio},
};

// impl Script for Expand {
//     fn handle_matches(matches: &ArgMatches) -> Result<(), CIError> {
//         let dest = PathBuf::from_str(r"target/expand/debug")
//             .expect("destination path");

//         expand(dest);

//         Ok(())
//     }
// }

pub(crate) fn expand(dest: PathBuf) {
    let is_rust_fmt = check_rustfmt();

    match std::fs::remove_dir_all(dest.to_owned()) {
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => panic!("Cannot remove destination, err: {}", err),
        Ok(_) => {
            log!("Removed destination, {:?}", dest);
        }
    }

    fs::create_dir_all(dest.to_owned())
        .expect("Destination should be re-created");

    for p in ["bin", "lib"] {
        fs::create_dir_all(dest.join(p)).unwrap_or_else(|err| {
            panic!("{} should be created, err: {}", p, err)
        });
    }

    let bin_path = dest.join("bin");
    let bin_crates = vec![vec!["saksaha", "sak"]];
    for b in bin_crates {
        let expand_bin_args = make_expand_bin_args(b[0], b[1]);
        execute_expand(
            b[0],
            b[1],
            bin_path.to_owned(),
            expand_bin_args,
            is_rust_fmt,
        );
    }

    let lib_path = dest.join("lib");
    let lib_crates = vec![vec!["saksaha", "lib"]];
    for l in lib_crates {
        let expand_lib_args = make_expand_lib_args(l[0]);
        execute_expand(
            l[0],
            l[1],
            lib_path.to_owned(),
            expand_lib_args,
            is_rust_fmt,
        );
    }
}

fn check_rustfmt() -> bool {
    let is_rust_fmt = match Cmd::new("rustfmt").output() {
        Ok(_) => {
            log!("rustfmt is found, will format expanded output");
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
    log!("Executing rustfmt on file: {:?}", file_path);

    Cmd::new("rustfmt")
        .arg(file_path.to_str().expect("file path must be stringified"))
        .output()
        .unwrap_or_else(|err| {
            panic!("Rustfmt should be executed, err: {}", err);
        });
}

fn execute_expand(
    pkg: &str,
    component: &str,
    dest: PathBuf,
    args: Vec<&str>,
    is_rust_fmt: bool,
) {
    let cmd = Cmd::new("cargo")
        .args(args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| {
            panic!("Command should be executed, bin: {}, err: {}", pkg, err)
        });

    let file_path = dest.join(format!("{}_{}.rs", pkg, component));
    let mut f = fs::File::create(file_path.to_owned()).unwrap_or_else(|err| {
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

fn make_expand_bin_args<'a>(pkg: &'a str, bin: &'a str) -> Vec<&'a str> {
    vec![
        "rustc",
        "-p",
        pkg,
        "--bin",
        bin,
        "--profile=check",
        "--",
        "-Zunpretty=expanded",
    ]
}

fn make_expand_lib_args<'a>(pkg: &'a str) -> Vec<&'a str> {
    vec![
        "rustc",
        "-p",
        pkg,
        "--lib",
        "--profile=check",
        "--",
        "-Zunpretty=expanded",
    ]
}
