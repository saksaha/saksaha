use super::{Disc, whoareyou::WhoAreYou};
use crate::{
    common::SakResult,
    err_res,
    p2p::{
        address::AddressBook,
        peer_store::{Peer, PeerStore},
    },
};
use logger::log;
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Mutex, MutexGuard},
    time,
};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    disc_port: usize,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: usize,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
        }
    }

    pub async fn start_dialing(&self) {
        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        loop {
            let start = SystemTime::now();

            let (addr, idx) = match self.address_book.next().await {
                Some(a) => a,
                None => {
                    println!("Addr not available");
                    time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };
            let addr = addr.lock().await;

            let peer = match self.peer_store.next().await {
                Some(p) => p,
                None => {
                    println!("Peer not available");
                    time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };

            println!("33 {:?}, {:?}", addr, peer);
            if addr.endpoint != my_disc_endpoint {
                let stream =
                    match TcpStream::connect(addr.endpoint.to_owned()).await {
                        Ok(s) => {
                            log!(
                                DEBUG,
                                "Successfully connected to endpoint, {}\n",
                                addr.endpoint
                            );
                            s
                        }
                        Err(err) => {
                            log!(
                                DEBUG,
                                "Error connecting to addr, {:?}, err: {}",
                                addr,
                                err
                            );
                            continue;
                        }
                    };

                let mut handler = Handler::new(stream, peer);
                handler.run().await;


                // let (mut rd, mut wr) = io::split(stream);

                // println!("31355");

                // wr.write_all(b"power\n").await.unwrap();

                // let mut buf = vec![0; 128];

                // println!("313");

                // loop {
                //     let n = rd.read(&mut buf).await.unwrap();

                //     if n == 0 {
                //         break;
                //     }

                //     println!("GOT {:?}", &buf[..n]);
                // }

                // println!("31344");

                // let h = Handler::new(stream);
                // h.run();
            } else {
                println!("!313");
                match self.address_book.remove(idx).await {
                    Ok(_) => (),
                    Err(err) => {
                        println!("err: {}", err);
                    }
                }
            }

            return;

            tokio::time::sleep(Duration::new(1, 0)).await;
            match start.elapsed() {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error sleeping the duration, err: {}", err);
                }
            }
        }
    }
}

pub struct Handler {
    stream: TcpStream,
    peer: Arc<Mutex<Peer>>,
}

impl Handler {
    pub fn new(stream: TcpStream, peer: Arc<Mutex<Peer>>) -> Handler {
        Handler {
            stream: stream,
            peer,
        }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        let buf = WhoAreYou::create();
        let v = [0; 1024];
        self.stream.write_all(&v);

        // let way = WhoAreYou::parse(&mut self.stream).await;
        Ok(true)
    }
}
