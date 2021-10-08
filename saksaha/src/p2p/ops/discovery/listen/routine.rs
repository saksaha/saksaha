use super::handler::Handler;
use crate::p2p::{
    credential::Credential,
    ops::discovery::listen::handler::HandleStatus,
    peer::peer_store::{Filter, PeerStore},
};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, sync::Mutex};

pub struct Routine {
    // address_book: Arc<AddressBook>,
    // peer_store: Arc<PeerStore>,
    // credential: Arc<Credential>,
}

impl Routine {
    pub fn new(
    ) -> Routine {
        Routine {}
    }

    pub fn run(&self,
        tcp_listener: TcpListener, peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) {
        log!(DEBUG, "Start listen - disc\n");

        let credential = credential.clone();
        let peer_store = peer_store.clone();

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
                        HandleStatus::NoAvailablePeerSlot => {
                            log!(DEBUG, "No available peer slot, sleeping");

                            tokio::time::sleep(Duration::from_millis(1000))
                                .await;
                        }
                        HandleStatus::PeerAlreadyTalking(endpoint) => {
                            log!(
                                DEBUG,
                                "Peer might be in talk already, endpoint: {}\n",
                                endpoint,
                            );
                        }
                        HandleStatus::AddressAcquireFail(err) => {
                            log!(
                                DEBUG,
                                "Cannot acquire address of \
                                incoming connection, err: {}\n",
                                err
                            );
                        }
                        HandleStatus::Success => (),
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
                    };
                });
            }
        });
    }
}
