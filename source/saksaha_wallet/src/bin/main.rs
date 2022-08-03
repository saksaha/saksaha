use saksaha_wallet::{App, WalletError};

fn main() -> Result<(), WalletError> {
    let pconfig = {};
    let app = App::init();

    app.run()?;

    Ok(())
}
