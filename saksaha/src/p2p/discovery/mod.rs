use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Disc {}

impl Disc {
    pub fn new() -> Self {
        Disc{}
    }
}

impl Disc {
    pub fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        println!("addr: {}", addr);

        for stream in listener.incoming() {
            println!("new\n");
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("request: {}", String::from_utf8_lossy(&buffer));
}

fn get_available_port() {
}
