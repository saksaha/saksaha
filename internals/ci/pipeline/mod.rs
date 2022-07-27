use crate::log;
use std::process::{Command as Cmd, Stdio};

pub(crate) fn build_3rd_party_contracts() {
    log!("build 3rd party contracts");
}

pub(crate) fn build_system_contracts() {
    log!("build system contracts");

    let mut a = Cmd::new("power");
    let b = a
        .args(["o"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    println!("power: {:?}", b);

    // .output();
    // .expect("failed to run");
}
