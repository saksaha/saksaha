use clap::{Arg, Command};
use sak_logger::RUST_LOG_ENV;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("Starting sak_wasm_postprocess");

    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }
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
        )
        .arg(
            Arg::new("multi-return-symbols")
                .long("multi-return-symbols")
                .multiple_values(true)
                .takes_value(true)
                .required(true),
        );

    let matches = app.get_matches();

    let output_path = {
        let output = matches.value_of("output");
        if let Some(p) = output {
            Some(PathBuf::from(p))
        } else {
            None
        }
    };

    let wasm_file = {
        let p = matches.value_of("file").expect("'File' should be provided");
        PathBuf::from(p)
    };

    let return_symbols = {
        let values = matches
            .values_of("multi-return-symbols")
            .expect("multi-return-symbols should be provided");

        values.map(|v| v.to_string()).collect()
    };

    postprocess_wasm_file(wasm_file, output_path, return_symbols);

    return ExitCode::SUCCESS;
}

fn postprocess_wasm_file(
    src_path: PathBuf,
    output_path: Option<PathBuf>,
    multi_return_symbols: Vec<String>,
) {
    if let Err(err) = wasm_postprocess::make_wasm_have_multiple_returns(
        src_path,
        output_path,
        multi_return_symbols,
    ) {
        println!("Error post procerssing wasm file, err: {}", err);
    }
}
