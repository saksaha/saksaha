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

        let handle = std::thread::spawn(|| {

            // everything in here runs
            // in its own separate thread
            for i in 0..10 {

                println!("Loop 2 iteration: {}", i);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        // main thread
        for i in 0..5 {

            println!("Loop 1 iteration: {}", i);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        handle.join().unwrap();

        // for i in 0..30 {
        //     self.tpool.execute(move |id| {
        //         handle_connection(i, id);
        //     });
        // }

        // for stream in listener.incoming() {
        //     println!("{:?}", stream);
        // }
    }
}

fn handle_connection(i: i32, id: usize) {
    println!("handle() i: {}, id: {}", i, id);

    // let mut buffer = [0; 1024];

    // stream.read(&mut buffer).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("handle done() i: {}, id: {}", i, id);

    // println!("request: {}", String::from_utf8_lossy(&buffer));
}

// fn handle_connection(id: usize, mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     println!("handle(): id: {}", id);

//     // stream.read(&mut buffer).unwrap();

//     std::thread::sleep(std::time::Duration::from_secs(3));

//     // println!("request: {}", String::from_utf8_lossy(&buffer));
// }
