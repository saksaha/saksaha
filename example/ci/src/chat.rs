use std::time::Duration;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
use hyper::Client;
use tokio::{io::stdin, runtime::Runtime};
use std::thread;
use std::io::{stdout, Write};

pub fn start_chat() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async {get_new_message().await});
    });

    loop{
        // send new messag
        // stdin()
    }
}

async fn get_new_message() {
    let mut stdout = stdout();
    // clear console log
    // set background color
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
        // SetBackgroundColor(Color::Magenta)
    );
    loop {
        let client = Client::new();
        let bootstrap = "http://google.com".parse::<hyper::Uri>().unwrap();
        let mut res = client.get(bootstrap).await;

        stdout.queue(cursor::SavePosition);
        stdout.write(
            format!("New chat {:?}", res.unwrap().body()).as_bytes(),
        );
        stdout.queue(cursor::RestorePosition);
        stdout.flush();
        thread::sleep(Duration::from_secs(1));
    }
}