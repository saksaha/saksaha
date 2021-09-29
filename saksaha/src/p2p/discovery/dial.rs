use std::sync::Arc;

use super::Disc;
use crate::{common::SakResult, err_res, p2p::peer_store::PeerStore};
use logger::log;
use tokio::{net::TcpStream, sync::Mutex};

pub struct Dial {
    pub address_book: Vec<String>,
    pub peer_store: Arc<Mutex<PeerStore>>,
    disc_port: usize,
}

impl Dial {
    pub fn new(
        bootstrap_peers: Option<Vec<String>>,
        peer_store: Arc<Mutex<PeerStore>>,
        disc_port: usize,
    ) -> Dial {
        let default_peers = crate::default_bootstrap_peers!()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let address_book = match bootstrap_peers {
            Some(b) => b,
            None => Vec::new(),
        };

        let address_book = [default_peers, address_book].concat();
        for (idx, addr) in address_book.iter().enumerate() {
            log!(DEBUG, "address book [{}]: {}\n", idx, addr);
        }

        Dial {
            address_book,
            peer_store,
            disc_port,
        }
    }

    pub async fn start_dialing(&self) {
        let address_book = &self.address_book;

        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        loop {
            for addr in address_book.iter() {
                let endpoint: Vec<&str> = addr.split("@").collect();
                let endpoint = match endpoint.get(1) {
                    Some(e) => e,
                    None => {
                        log!(
                            DEBUG,
                            "Cannot get endpoint out of url. \
                            Something might be wrong\n"
                        );
                        continue;
                    }
                };

                if *endpoint == my_disc_endpoint {
                    continue;
                }

                let stream = TcpStream::connect(endpoint).await;

                let h = Handler::new();
                h.run();
            }
        }
    }
}

struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn run(&self) {

    }
}

#[macro_export]
macro_rules! default_bootstrap_peers {
    () => {
        vec!["sak://041efae14ece202c@127.0.0.1:35518"]
    };
}
