use super::handler::Handler;
use crate::p2p::{address::{Address, address_book::AddressBook}, credential::Credential, discovery::listen::handler::HandleStatus, peer::peer_store::PeerStore};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;

pub struct Routine {
    address_book: Arc<AddressBook>,
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
}

impl Routine {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) -> Routine {
        Routine {
            address_book,
            peer_store,
            credential,
        }
    }

    pub async fn run(&self, tcp_listener: TcpListener, peer_op_port: u16) {
        log!(DEBUG, "Start disc listening\n");

        loop {
            let peer_store = self.peer_store.clone();
            let peer = match peer_store.next().await {
                Some(p) => p,
                None => {
                    log!(DEBUG, "No available peer\n");

                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    continue;
                }
            };

            let addr = match self.address_book.reserve().await {
                Some(a) => a,
                None => {
                    log!(DEBUG, "No available address slot\n");

                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    continue;
                }
            };

            // Address::new();

            // let addr = self.address_book.next();

            let (stream, peer_addr) = match tcp_listener.accept().await {
                Ok(res) => {
                    log!(DEBUG, "Accepted incoming request, addr: {}\n", res.1);
                    res
                }
                Err(err) => {
                    log!(DEBUG, "Error accepting request, err: {}", err);
                    continue;
                }
            };

            let credential = self.credential.clone();

            let address_book = self.address_book.clone();

            let mut handler = Handler::new(
                address_book,
                stream,
                peer.clone(),
                credential,
                peer_op_port,
            );

            tokio::spawn(async move {
                match handler.run().await {
                    HandleStatus::Success => {
                        return;
                    }
                    HandleStatus::WhoAreYouReceiveFail(err) => {
                        log!(
                            DEBUG,
                            "Disc listen failed receiving \
                                who are you, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::WhoAreYouAckInitiateFail(err) => {
                        log!(
                            DEBUG,
                            "Disc listen failed initiating \
                                who are you ack, err: {}\n",
                            err
                        );
                    }
                    HandleStatus::PeerUpdateFail(err) => {
                        log!(
                            DEBUG,
                            "Disc listen failed updating peer, err: {}\n",
                            err
                        );
                    }
                }
            });
        }
    }
}
