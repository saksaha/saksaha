mod cli;

use std::{thread, time::Duration};

use saksaha_wallet::{App, AppArgs, WalletError};

const RUST_LOG_ENV: &str = "
    sak_,
    saksaha
";

fn main() -> Result<(), WalletError> {
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args = cli::get_args()?;

    let app_args = AppArgs {
        rpc_port: cli_args.rpc_port,
        public_key: cli_args.public_key,
        secret: cli_args.secret,
    };

    let app = App::init();

    // use std::io::BufRead;
    // let stdin = std::io::stdin();

    // loop {
    //     println!("11l");

    //     for line in stdin.lock().lines() {
    //         println!("44, {}", line.unwrap());
    //     }

    //     thread::sleep(Duration::from_secs(1));
    // }

    // println!("should be waiting!!! 123123");

    app.run(app_args)?;

    Ok(())
}
