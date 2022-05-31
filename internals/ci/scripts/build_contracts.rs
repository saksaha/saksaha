use crate::log;
use crate::script::Script;
use crate::scripts::BoxedError;
use clap::ArgMatches;
use lazy_static::lazy_static;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::process::Command as Cmd;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref CONTRACTS: Vec<DirEntry> = {
        let project_root = match std::env::var("PROJECT_ROOT") {
            Ok(r) => PathBuf::from(r),
            Err(err) => {
                log!("Env (PROJECT_ROOT) is not given. Did you run in \
                    from 'ci'?, err: {}", err);

                std::process::exit(1);
            }
        };

        let contracts_dir = {
            let p = project_root.join("source")
                .join("saksaha/src/contracts");

            if p.exists() {
                match std::fs::read_dir(p) {
                    Ok(d) => d,
                    Err(err) => {
                        log!("Contract directory needs to be loaded, err: {}",
                            err);

                        std::process::exit(1);
                    }
                }
            } else {
                log!("Contract directory does not exists");

                std::process::exit(1);
            }
        };

        let contracts: Vec<DirEntry> = contracts_dir.map(|f| {
            f.expect("Contents in the directory should be read")
        }).collect();

        contracts
    };

    static ref EXAMPLE: u8 = 42;
}

pub(crate) struct BuildContracts;

impl Script for BuildContracts {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        for elem in CONTRACTS.iter() {
            let path = match elem.path().into_os_string().into_string() {
                Ok(p) => p,
                Err(err) => {
                    log!(
                        "path of a contract should be resolved, err: {}",
                        err.to_string_lossy()
                    );

                    std::process::exit(1);
                }
            };

            let args = ["build", &path];

            let cmd = Cmd::new("wasm-pack")
                .args(args)
                .spawn()
                .expect("failed to run");

            cmd.wait_with_output().unwrap();
        }

        Ok(())
    }
}