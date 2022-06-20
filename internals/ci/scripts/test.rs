use crate::scripts::BoxedError;
use crate::{log, script::Script};
use clap::ArgMatches;
use std::env;
use std::path::PathBuf;
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Test;

impl Script for Test {
    fn handle_matches(matches: &ArgMatches) -> Result<(), BoxedError> {
        let program = "cargo";

        let args = match matches.values_of("SAKSAHA_ARGS") {
            Some(a) => a.collect::<Vec<&str>>().join(" "),
            None => String::new(),
        };

        let args =
            vec!["test", &args[..], "--", "--nocapture", "--show-output"];

        log!("Executing `{} {:?}`", program, args);

        {
            let project_root = env::var("PROJECT_ROOT")
                .expect("PROJECT_ROOT should be provided");

            let system_contracts = vec!["sak_ctrt_validator"];

            let sys_con_paths = {
                let mut v = Vec::with_capacity(system_contracts.len());

                for c in system_contracts {
                    let contract_path =
                        PathBuf::from(&project_root).join("source").join(c);

                    let contract_path =
                        contract_path.to_string_lossy().to_string();

                    v.push(contract_path);
                }

                v.join(",")
            };

            env::set_var("SYS_CONTRACT_PATH", sys_con_paths);
        }

        Cmd::new(program)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");

        Ok(())
    }
}
