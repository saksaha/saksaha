use logger::log;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use super::handler::Handler;
use crate::p2p::{
    address::AddressBook, credential::Credential,
    discovery::dial::handler::HandleResult, peer::peer_store::PeerStore,
};

pub struct Routine {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
    address_book: Arc<AddressBook>,
    peer_op_port: u16,
    disc_port: u16,
}

impl Routine {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        address_book: Arc<AddressBook>,
        peer_op_port: u16,
        disc_port: u16,
    ) -> Routine {
        Routine {
            peer_store,
            credential,
            address_book,
            peer_op_port,
            disc_port,
        }
    }

    pub async fn run(&self) {
        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        loop {
            let start = SystemTime::now();

            if let Some(peer) = self.peer_store.next().await {
                let credential = self.credential.clone();
                let address_book = self.address_book.clone();

                let mut handler = Handler::new(
                    peer,
                    credential,
                    self.peer_op_port,
                    address_book,
                    my_disc_endpoint.to_owned(),
                );

                match handler.run().await {
                    Ok(res) => {
                        if let HandleResult::AddressNotFound = res {
                            break;
                        }
                    }
                    Err(err) => {
                        log!(DEBUG, "Error processing request, err: {}\n", err,);
                    }
                }
            } else {
                log!(DEBUG, "Peer not available");

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }

            tokio::time::sleep(Duration::from_millis(1000)).await;

            match start.elapsed() {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error sleeping the duration, err: {}", err);
                }
            }
        }

        // match dial_loop_rx.recv().await {
        //     Some(_) => (),
        //     None => {
        //         let msg = msg_err!(
        //             MsgKind::ResourceNotAvailable,
        //             "dial loop channel has been closed",
        //         );

        //         self.task_mng.send(msg).await;
        //     }
        // }
    }
}
