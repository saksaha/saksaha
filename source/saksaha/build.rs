use chrono::Utc;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

type BoxedError = Box<dyn std::error::Error>;

fn main() {
    let build_path = clean_build_dir();

    let log_file = open_log_pipe();

    build_sys_contracts(log_file, build_path)
        .expect("System contracts should be pre-built.");
}

fn clean_build_dir() -> PathBuf {
    let curr_dir = env::current_dir().unwrap();

    let build_path = curr_dir.join("build");

    for entry in fs::read_dir(&build_path).expect("build should exist") {
        let entry = entry.unwrap();
        if entry.file_name() != ".gitkeep" {
            fs::remove_file(entry.path()).expect("File should be deleted");
        }
    }

    return build_path;
}

fn open_log_pipe() -> File {
    let mut output = File::create("build/build_output")
        .expect("build_output should be created.");

    write!(output, "Build at: {:?}\n\n", Utc::now())
        .expect("File should be writable");

    output
}

fn build_sys_contracts(
    mut log_file: File,
    build_path: PathBuf,
) -> Result<(), BoxedError> {
    let sys_con_paths = env::var("SYS_CONTRACT_PATH")
        .expect("System contract paths have to be provided");

    let sys_con_paths: Vec<&str> = sys_con_paths.split(",").collect();
    let sys_con_paths_len = sys_con_paths.len();

    for (idx, sys_con_path) in sys_con_paths.iter().enumerate() {
        writeln!(
            log_file,
            "[{}/{}] building system contract, path: {}",
            idx + 1,
            sys_con_paths_len,
            sys_con_path,
        )?;

        if let Err(err) =
            build_sys_con(&mut log_file, sys_con_path, &build_path)
        {
            panic!("Build system contract fail, err: {}", err);
        }
    }

    Ok(())
}

fn build_sys_con(
    log_file: &mut File,
    sys_con_path: &str,
    build_path: &PathBuf,
) -> Result<(), BoxedError> {
    let sys_con_path = PathBuf::from(sys_con_path);

    let program = "cargo";

    let args = [vec!["wasm"]].concat();

    writeln!(
        log_file,
        "Building system contract, {} {}, current_dir: {:?}`",
        program,
        args.join(" "),
        &sys_con_path,
    )?;

    let sys_con_target = sys_con_path.join("target");
    let sys_con_name = sys_con_path.file_name().unwrap().to_str().unwrap();

    Command::new(program)
        .current_dir(&sys_con_path)
        .args(&["wasm", "--target-dir", sys_con_target.to_str().unwrap()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    let wasm_file = sys_con_target.join(format!(
        "wasm32-unknown-unknown/release/{}.wasm",
        sys_con_name,
    ));

    post_process_wasm(log_file, &wasm_file, build_path)?;

    writeln!(log_file, "System_contract ({:?}) has been built", wasm_file,)?;

    Ok(())
}

fn post_process_wasm(
    log_file: &mut File,
    wasm_path: &PathBuf,
    build_path: &PathBuf,
) -> Result<(), BoxedError> {
    writeln!(
        log_file,
        "post_process_wasm(): wasm_path: {:?}, build_path: {:?}",
        wasm_path, build_path,
    )?;

    let file_name = wasm_path.file_name().unwrap();

    let output_path = build_path.join(file_name);

    sak_wasm_postprocess::make_wasm_have_multiple_returns(
        wasm_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
    );

    Ok(())
}
