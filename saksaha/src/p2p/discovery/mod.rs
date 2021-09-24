mod listen;
mod dial;

use tokio::net::TcpListener;
use logger::log;
use crate::{common::SakResult, err_res};

pub struct Disc {
    disc_port: usize,
}

impl Disc {
    pub fn new(disc_port: usize, bootstrap_peers: Vec<String>) -> Self {
        Disc { disc_port }
    }
}

impl Disc {
    pub async fn start(&self) -> SakResult<TcpListener> {
        return self.start_listening().await;
    }
}

// fn handle_connection(id: usize, mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";
//     let sleep = b"GET /sleep HTTP/1.1\r\n";

//     let (a, b) = if buffer.starts_with(get) {
//         println!("get");
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else if buffer.starts_with(sleep) {
//         println!("sleep");
//         std::thread::sleep(std::time::Duration::from_secs(20));
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         println!("not defined");
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let contents = "response\n";
//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
