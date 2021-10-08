mod handler;
mod status;

use crate::{
    common::{Error, Result},
    msg_err,
    node::task_manager::TaskManager,
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use handler::Handler;
use logger::log;
pub use status::Status;
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, sync::Mutex};

use self::handler::HandleStatus;

pub struct Listener {}

impl Listener {
    pub fn new() -> Listener {
        Listener {}
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) -> Status<u16, Error> {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => {
                        // log!(DEBUG, "Listener created, addr: {}\n", local_addr);

                        (listener, local_addr)
                    }
                    Err(err) => return Status::SetupFailed(err.into()),
                },
                Err(err) => return Status::SetupFailed(err.into()),
            };

        log!(DEBUG, "Started - Disc listener, addr: {}\n", local_addr);

        let routine = Routine::new();
        routine.run(tcp_listener, p2p_listener_port, peer_store, credential);

        Status::Launched(local_addr.port())
    }
}

struct Routine {}

impl Routine {
    pub fn new() -> Routine {
        Routine {}
    }

    pub fn run(
        &self,
        tcp_listener: TcpListener,
        peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) {
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
