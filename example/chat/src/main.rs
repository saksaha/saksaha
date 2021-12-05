mod chat;
mod ledger;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    ci_type: String,
    /// The chat room id
    meta_property: Option<String>,
}

const LEDGER: &str = "ledger";
const CHAT: &str = "chat";

// #[tokio::main]
fn main() {
    println!("start chat application");

    let args = Cli::from_args();
    println!("trying to init  {:?}", args.ci_type);

    match args.ci_type.as_str() {
        LEDGER => ledger::show_ledger(),
        CHAT => chat::start_chat(),
        _ => println!("args not provided"),
    }

    loop {}
    println!("program exit");
}
