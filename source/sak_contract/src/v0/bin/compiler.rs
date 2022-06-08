use clap::{arg, command};
use std::process::Command;

pub fn main() {
    let matches = command!()
        .arg(
            arg!([wasm_path] "Contract(project) path to compile")
                .required(true),
        )
        .get_matches();

    if let Some(p) = matches.value_of("contract_path") {
        Command::new("cargo")
            .env("CARGO_TARGET_DIR", p)
            .args(&["--pkg", p, "target", "wasm32-wasi"])
            .spawn()
            .unwrap();
    }
}
