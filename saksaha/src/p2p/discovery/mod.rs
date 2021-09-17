use crate::thread::ThreadPool;
use logger::log;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

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
            let stream = stream.unwrap();

            self.tpool.execute(|id| {
                handle_connection(id, stream);
            });
        }
    }
}

fn handle_connection(id: usize, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("handle() id: {}", id);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let () = if buffer.starts_with(get) {
        println!("1");
        // format!("HTTP/1.1 200 OK", "hello.html");
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("2");
        // ("HTTP/1.1 200 OK", "hello.html")
    } else {
        println!("3");
        // ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
}

// fn handle_connection(id: usize, mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     println!("handle(): id: {}", id);

//     // stream.read(&mut buffer).unwrap();

//     std::thread::sleep(std::time::Duration::from_secs(3));

//     // println!("request: {}", String::from_utf8_lossy(&buffer));
// }
