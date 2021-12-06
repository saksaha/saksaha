use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::sync::Mutex;
use tokio::{self, signal};

use crate::chat::Chat;
use crate::data::{client_0, client_1};

pub struct ChatApp {
    cid: String,
}

impl ChatApp {
    pub fn new(cid: String) -> ChatApp {
        ChatApp { cid }
    }

    pub fn run(&self) {
        println!("Chat app run, cid: {}", self.cid);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                start_routine(self.cid.clone()).await;

                tokio::select!(
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                println!("ctrl+k is pressed.");

                std::process::exit(1);
                            },
                            Err(err) => {
                                println!(
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                std::process::exit(1);

                            }
                        }
                    },
                );
            }),
            Err(err) => {
                std::process::exit(1);
            }
        }
    }
}

async fn start_routine(cid: String) {
    if cid.clone() == "0" {
        let my_ip = client_0::IP;
        let my_port = client_0::PORT;

        let local_addr = format!("{}:{}", my_ip, my_port);
        let client_id = cid.clone();

        tokio::spawn(async move {
            let cid = client_id.clone();

            let tcp_listener = match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => {
                        println!(
                            "Listener bound the address, addr: {}",
                            local_addr
                        );
                        listener
                    }
                    Err(err) => {
                        println!(
                            "Can't get local address of tcp listener, err: {}",
                            err
                        );
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    println!("Can't bind tcp listener, err: {}", err,);
                    std::process::exit(1);
                }
            };

            let stream = match tcp_listener.accept().await {
                Ok((stream, addr)) => {
                    println!("Accepted new conneciton, addr: {:?}", addr);

                    stream
                }
                Err(err) => {
                    println!(
                        "Error accepting connection request, err: {}",
                        err,
                    );
                    std::process::exit(1);
                }
            };

            let mut chat = Chat {
                cid: cid.clone(),
                stream,
            };
            chat.start().await;
        });
    }

    if cid == "1" {
        let dst_id = client_0::IP;
        let dst_port = client_0::PORT;

        let endpoint = format!("{}:{}", dst_id, dst_port);

        println!("Connecting endpoint: {}...", endpoint);

        let stream = match TcpStream::connect(&endpoint).await {
            Ok(s) => {
                println!("Connected to addr, {:?}", s.peer_addr());
                s
            }
            Err(err) => {
                println!(
                    "Cannot connect to client, cid: {}, err: {}",
                    cid.clone(),
                    err
                );

                std::process::exit(1);
            }
        };

        let mut chat = Chat {
            cid: cid.clone(),
            stream,
        };
        chat.start().await;
    }
}
