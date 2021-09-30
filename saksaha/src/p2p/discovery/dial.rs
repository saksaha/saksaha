use std::{sync::Arc, thread, time::{Duration, SystemTime}};

use super::Disc;
use crate::{common::SakResult, err_res, p2p::{address::AddressBook, peer_store::{Peer, PeerStore}}};
use logger::log;
use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}, time};

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
                    time::sleep(Duration::from_millis(1000));
                    continue;
                }
            };
            let addr = addr.lock().await;

            let peer = match self.peer_store.next().await {
                Some(p) => p,
                None => {
                    println!("Peer not available");
                    time::sleep(Duration::from_millis(1000));
                    continue;
                }
            };

            let peer = peer.lock().await;

            println!("33 {:?} {}", addr, addr.endpoint == my_disc_endpoint);
            if addr.endpoint != my_disc_endpoint {
                println!("444");
                let stream = TcpStream::connect(addr.endpoint.to_owned()).await;

                // let h = Handler::new(stream);
                // h.run();
            } else {
                self.address_book.remove(idx);
            }

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

pub struct Handler<'a> {
    stream: TcpStream,
    peer: MutexGuard<'a, Peer>,
}

impl<'a> Handler<'a> {
    pub fn new(stream: TcpStream, peer: MutexGuard<'a, Peer>) -> Handler<'a> {
        Handler { stream: stream, peer }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        // let way = WhoAreYou::parse(&mut self.stream).await;
        Ok(true)
    }
}
