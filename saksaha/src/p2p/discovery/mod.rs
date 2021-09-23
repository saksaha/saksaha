use tokio::net::TcpListener;
use logger::log;
// use crate::sync::ThreadPool;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
use crate::{common::SakResult, err_res, sync::ThreadPool};


pub struct Disc {
    // pub tpool: ThreadPool,
}

impl Disc {
    pub fn new(bootstrap_peers: Vec<String>) -> Self {
        Disc { }
    }
}

impl Disc {
    pub async fn start(&self) -> SakResult<TcpListener> {
        // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        // let addr = listener.local_addr().unwrap();

        // println!("addr: {}", addr);

        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();

        //     self.tpool.execute(|id| {
        //         handle_connection(id, stream);

        //         None
        //     });
        // }

        let listener = match TcpListener::bind("127.0.0.1:8080").await {
            Ok(l) => (l),
            Err(_) => {
                return err_res!("Error start listeneing");
            },
        };

        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return err_res!("Error accepting a request, err: {}", err);
                }
            };

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // let n = match
                }
            });
        }

        //     Ok(l) => l,
        //     Err(err) => {
        //         return err_res!("Error start listening");
        //     }
        // };
        return Ok(listener);
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
