use std::sync::Arc;

use super::Disc;
use crate::{common::SakResult, err_res, p2p::peer_store::PeerStore};
use logger::log;
use tokio::{net::TcpStream, sync::Mutex};

pub struct Dial {
    pub address_book: Vec<String>,
    pub peer_store: Arc<Mutex<PeerStore>>,
}

impl Dial {
    pub fn new(
        bootstrap_peers: Option<Vec<String>>,
        peer_store: Arc<Mutex<PeerStore>>,
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

        Dial {
            address_book,
            peer_store,
        }
    }

    pub async fn start_dialing(&self) -> SakResult<bool> {
        let address_book = &self.address_book;

        for addr in address_book.iter() {
            let endpoint: Vec<&str> = addr.split("@").collect();
            let endpoint = match endpoint.get(1) {
                Some(e) => e,
                None => {
                    log!(DEBUG, "Cannot get endpoint out of url. \
                        Something might be wrong");
                    continue;
                }
            };

            let stream = TcpStream::connect(endpoint).await;

            // let h = Handler::new();
            // h.run();
        }

        // let stream = TcpStream::connect()
        // log!(DEBUG, "Start disc dialing\n");

        Ok(true)
    }
}

struct Handler {

}

impl Handler {

}


#[macro_export]
macro_rules! default_bootstrap_peers {
    () => {
        vec!["sak://041efae14ece202c@127.0.0.1:35518"]
    };
}
