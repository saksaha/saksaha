use std::sync::Arc;

use super::Disc;
use crate::{
    common::SakResult,
    err_res,
    p2p::{address::AddressBook, peer_store::PeerStore},
};
use logger::log;
use tokio::{net::TcpStream, sync::Mutex};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<Mutex<PeerStore>>,
    disc_port: usize,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<Mutex<PeerStore>>,
        disc_port: usize,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
        }
    }

    pub async fn start_dialing(&self) {
        // let address_book = &self.address_book;

        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        // loop {
        //     for addr in address_book.iter() {
        //         let endpoint: Vec<&str> = addr.split("@").collect();
        //         let endpoint = match endpoint.get(1) {
        //             Some(e) => e,
        //             None => {
        //                 log!(
        //                     DEBUG,
        //                     "Cannot get endpoint out of url. \
        //                     Something might be wrong\n"
        //                 );
        //                 continue;
        //             }
        //         };

        //         if *endpoint == my_disc_endpoint {
        //             continue;
        //         }

        //         let stream = TcpStream::connect(endpoint).await;

        //         let h = Handler::new();
        //         h.run();
        //     }
        // }
    }
}

struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn run(&self) {}
}
