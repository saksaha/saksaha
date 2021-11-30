use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
use hyper::Client;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

enum ChatStatus {
    Idle,
    ChatList,
    ChatStart,
}

pub fn start_chat() {
    let mut status = ChatStatus::Idle;
    let mut rt = Runtime::new().unwrap();

    // Do init here
    status = ChatStatus::ChatList;

    loop {
        match status {
            ChatStatus::ChatList => {
                rt.block_on(async {
                    get_chat_list().await;
                });
                println!("choose chat room number");
                let mut chat_room_id = String::new();
                match stdin().read_line(&mut chat_room_id) {
                    Ok(_) => status = ChatStatus::ChatStart,
                    Err(err) => println!("invalid chat_room_id {}", err),
                }
            }
            ChatStatus::ChatStart => {
                rt.block_on(async move {
                    tokio::spawn(async { get_new_message().await });
                });

                loop {
                    // send new messag
                    let mut buffer = String::new();
                    stdin().read_line(&mut buffer).expect("invalid message");
                }
            }
            ChatStatus::Idle => {
                // need init again
            }
        }
    }
}

async fn get_chat_list() {
    let mut stdout = stdout();
    let client = Client::new();
    let bootstrap = "http://google.com".parse::<hyper::Uri>().unwrap();
    let mut res = client.get(bootstrap).await;

    stdout.write(format!("chat list {:?}", res.unwrap().body()).as_bytes());
}

async fn get_new_message() {
    let mut stdout = stdout();
    // clear console log
    // set background color
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0) // SetBackgroundColor(Color::Magenta)
    );
    loop {
        let client = Client::new();
        let bootstrap = "http://google.com".parse::<hyper::Uri>().unwrap();
        let mut res = client.get(bootstrap).await;

        stdout.queue(cursor::MoveTo(0,0));
        stdout.queue(cursor::SavePosition);
        stdout.write(format!("New chat {:?}", res.unwrap().body()).as_bytes());
        // should move cursor under current chat
        stdout.queue(cursor::MoveDown(2));
        stdout.queue(cursor::MoveLeft(255));
        stdout.write(format!("Enter new message: ").as_bytes());
        stdout.flush();

        thread::sleep(Duration::from_secs(1));
    }
}
