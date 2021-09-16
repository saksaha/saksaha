use crate::thread::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use logger::log;

pub struct Disc {
    tpool: ThreadPool,
}

impl Disc {
    pub fn new(tpool: ThreadPool, bootstrap_peers: Vec<String>) -> Self {
        Disc { tpool }
    }
}

impl Disc {
    pub fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        println!("addr: {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    self.tpool.execute(|| {
                        handle_connection(s);
                    })
                },
                Err(ref e) if e.kind () == std::io::ErrorKind::WouldBlock => {
                    break;
                    // continue;
                },
                Err(err) => {
                    log!(DEBUG, "Error, err: {}", err);
                },
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("request: {}", String::from_utf8_lossy(&buffer));
}

fn get_available_port() {}
