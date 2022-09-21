use crate::WalletError;
use std::io::BufRead;

pub(crate) fn run() -> Result<bool, WalletError> {
    println!(
        "\nEither public_key or secret is empty. Would you want to \
        proceed to create a new credential? (y)es, (n)o:",
    );

    let stdin = std::io::stdin();

    let mut buf = String::new();
    let _ = stdin.lock().read_line(&mut buf);

    let buf = buf.to_lowercase();
    let user_input = buf.trim();

    let make_credential = match user_input {
        "y" => {
            println!("We will generate a new wallet credential...");
            true
        }
        "n" => {
            println!(
                "You have to manually create a wallet credentail and then \
                provide the credential. Exiting..."
            );

            std::process::exit(0);
        }
        _ => {
            println!("Invalid input. Terminating process.");

            std::process::exit(0);
        }
    };

    Ok(make_credential)
}
