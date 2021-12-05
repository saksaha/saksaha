use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
use hyper::Client;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn show_ledger() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async move {
        println!("get cur status");
        tokio::spawn(async { get_cur_status().await });
    });

    loop {}
}

async fn get_cur_status() {
    let mut stdout = stdout();
    // clear console log
    // set background color
    execute!(
        stdout,
        Clear(ClearType::All),
        SetBackgroundColor(Color::Magenta)
    );
    loop {
        let client = Client::new();
        let bootstrap = "http://google.com".parse::<hyper::Uri>().unwrap();
        let mut res = client.get(bootstrap).await;

        // set cursor to top left
        execute!(stdout, cursor::MoveTo(0, 0));

        stdout.queue(cursor::SavePosition);
        stdout.write(
            format!("Ledger Status {:?}", res.unwrap().body()).as_bytes(),
        );
        stdout.queue(cursor::RestorePosition);
        stdout.flush();
        thread::sleep(Duration::from_secs(1));
    }
}
