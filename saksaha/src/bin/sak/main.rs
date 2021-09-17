use clap::{App, Arg};
use logger::log;
use saksaha::{node::Node, p2p::host::Host, pconfig::PConfig};

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

fn main() {
    let flags = App::new("Saksaha rust")
        .version("0.1")
        .author("Saksaha <team@saksaha.com>")
        .about("Saksaha node rust client")
        .license("MIT OR Apache-2.0")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about(
                    "Saksaha configuration file, usually created at \
                    [[OS default config path]]/saksaha/config.json",
                ),
        )
        .arg(
            Arg::new("bootstrap_peers")
                .long("bootstrap-peers")
                .value_name("ENDPOINT")
                .use_delimiter(true)
                .about("Bootstrap peers to start discovery for"),
        )
        .arg(
            Arg::new("rpc_port")
                .long("rpc-port")
                .value_name("PORT")
                .about("RPC port number"),
        )
        .get_matches();

    let pconf = make_pconfig(flags.value_of("config"));

    let node = match Node::new(
        flags.value_of("rpc_port"),
        flags.values_of("bootstrap_peers"),
        pconf.p2p.public_key,
        pconf.p2p.secret,
    ) {
        Ok(n) => n,
        Err(err) => {
            log!(DEBUG, "Error creating a node, err: {}\n", err);
            std::process::exit(1);
        }
    };

    node.start();
}

// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let job = receiver.lock().unwrap().recv().unwrap();

//             println!("Worker {} got a job; executing.", id);

//             job();

//         });

//         Worker { id, thread }
//     }
// }

// type Job = Box<dyn FnOnce() + Send + 'static>;

// struct ThreadPool {
//     sender: mpsc::Sender<Job>,
//     workers: Vec<Worker>,
// }

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         let (sender, receiver) = mpsc::channel();

//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);

//         self.sender.send(job).unwrap();
//     }
// }

fn make_pconfig(config_path: Option<&str>) -> PConfig {
    let pconf = match PConfig::of(config_path) {
        Ok(p) => p,
        Err(err) => {
            log!(
                DEBUG,
                "Error creating a persisted configuration, err: {}\n",
                err
            );
            std::process::exit(1);
        }
    };

    log!(DEBUG, "Successfully loaded config, {:?}\n", pconf);
    pconf
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";
//     let sleep = b"GET /sleep HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else if buffer.starts_with(sleep) {
//         std::thread::sleep(std::time::Duration::from_secs(5));
//         ("HTTP/1.1 200 OK sleep", "hello.html")
//     }else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let contents = format!("123123 {}", status_line);

//     let response = format!(
//         "{}\r\nContent-Length: {}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
