mod paths;
mod scripts;
mod tasks;
mod utils;

use crate::{
    paths::Paths,
    scripts::{build, build_contracts, clean, dev, dev_evl_term, test},
};

pub(crate) type CIError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), CIError> {
    validate_curr_dir()?;

    run_script()?;

    Ok(())
}

fn validate_curr_dir() -> Result<(), CIError> {
    let curr_dir = std::env::current_dir()?;

    let curr_dir_str = match std::env::current_dir() {
        Ok(cd) => cd
            .into_os_string()
            .into_string()
            .expect("current dir must be stringified"),
        Err(_) => {
            log!("Cannot get current working directory");

            std::process::exit(1);
        }
    };

    let project_root = match std::env::var("PROJECT_ROOT") {
        Ok(p) => p,
        Err(_) => {
            log!(
                "PROJECT_ROOT is not defined. This is most likely due to \
                that this binary has not been executed from the project root. \
                Use `ci` script in the project. Exiting."
            );

            std::process::exit(1);
        }
    };

    log!(
        "CI is starting, project root: {}, current working directory: {}",
        project_root,
        curr_dir_str,
    );

    if project_root != curr_dir_str {
        log!(
            "Warning! You may be running this script not from the project \
            root with `ci` script"
        );
    }

    Paths::init(curr_dir)?;

    Ok(())
}

fn run_script() -> Result<(), CIError> {
    let mut args = std::env::args();

    let second_arg = args
        .nth(1)
        .expect("CI needs a second argument, the name of script to run");

    log!("script name (2nd arg): {:?}", second_arg);

    match second_arg.as_str() {
        "dev" => {
            dev::run(args)?;
        }
        "build" => {
            build::run(args)?;
        }
        "test" => {
            test::run(args)?;
        }
        "clean" => {
            clean::run(args)?;
        }
        "dev_evl_term" => {
            dev_evl_term::run(args)?;
        }
        "build_contracts" => {
            build_contracts::run(args)?;
        }
        _ => {
            return Err(format!(
                "Could not find the script of name: {}",
                second_arg,
            )
            .into());
        }
    };

    Ok(())
}
