use super::handler::Handler;
use crate::p2p::{
    credential::Credential,
    discovery::listen::handler::HandleStatus,
    peer::peer_store::{Filter, PeerStore},
};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;

pub struct Routine {
    // address_book: Arc<AddressBook>,
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
}

impl Routine {
    pub fn new(
        // address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) -> Routine {
        Routine {
            // address_book,
            peer_store,
            credential,
        }
    }

    pub fn run(&self, tcp_listener: TcpListener, peer_op_port: u16) {
        log!(DEBUG, "Start disc listening\n");

        let credential = self.credential.clone();
        let peer_store = self.peer_store.clone();

        tokio::spawn(async move {
            loop {
                let (stream, _) = match tcp_listener.accept().await {
                    Ok(res) => {
                        log!(
                            DEBUG,
                            "Accepted incoming request, addr: {}\n",
                            res.1
                        );
                        res
                    }
                    Err(err) => {
                        log!(DEBUG, "Error accepting request, err: {}", err);
                        continue;
                    }
                };

                let credential = credential.clone();
                let peer_store = peer_store.clone();
                let mut handler = Handler::new(
                    stream,
                    peer_store,
                    credential.clone(),
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
                        HandleStatus::JoinError(err) => {
                            log!(
                                DEBUG,
                                "Error joining a handler thread, err: {}\n",
                                err
                            );
                        }
                    };
                });
            }

        });
    }
}
