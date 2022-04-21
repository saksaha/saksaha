mod app;
mod chat;
mod data;

use crate::app::ChatApp;
use clap::{App, Arg};

fn main() {
    println!("Start Saksaha chat");

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("client_id")
                .long("client_id")
                .value_name("ID")
                .takes_value(true),
        )
        .get_matches();

    if let Some(cid) = matches.value_of("client_id") {
        println!("Client id [0, 1]: {}", cid);

        if cid == "0" || cid == "1" {
            let app = ChatApp::new(cid.to_string());
            app.run();
        } else {
            println!("Invalid client id, cid: {}", cid);
        }
    } else {
        println!("No client id is provided. Exiting..");
    }
}
