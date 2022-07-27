use clap::{Arg, Command};
use log::info;
use std::{os::unix::prelude::ExitStatusExt, process::ExitCode};

const RUST_LOG_ENV: &str = "
    sak_,
    saksaha
";

fn main() -> ExitCode {
    println!("Starting sak_wasm_postprocess");

    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        sak_logger::init(false);
    }

    let app = Command::new("Sak Wasm post processor")
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sak Wasm post processor")
        .arg(
            Arg::new("output")
                .long("output")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let output_path = matches
        .value_of("output")
        .expect("Output should be provided");

    let wasm_file =
        matches.value_of("file").expect("'File' should be provided");

    postprocess_wasm_file(wasm_file, output_path);

    return ExitCode::SUCCESS;
}

fn postprocess_wasm_file(file: &str, output_path: &str) {
    sak_wasm_postprocess::make_wasm_have_multiple_returns(file, output_path);
}
