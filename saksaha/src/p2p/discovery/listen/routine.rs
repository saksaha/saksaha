use super::handler::Handler;
use crate::p2p::{credential::Credential, peer::peer_store::PeerStore};
use logger::log;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Routine {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
}

impl Routine {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
    ) -> Routine {
        Routine { peer_store, credential }
    }

    pub async fn run(&self, tcp_listener: TcpListener, peer_op_port: u16) {
        loop {
            println!("start listen loop");

            let peer_store = self.peer_store.clone();

            if let Some(peer) = peer_store.next().await {
                let (stream, addr) = match tcp_listener.accept().await {
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

                let credential = self.credential.clone();

                tokio::spawn(async move {
                    let mut handler = Handler::new(
                        stream,
                        peer.clone(),
                        credential,
                        peer_op_port,
                    );

                    match handler.run().await {
                        Ok(_) => (),
                        Err(err) => {
                            log!(
                                DEBUG,
                                "Error processing request, addr: {}, err: {}",
                                addr,
                                err
                            );
                        }
                    }
                });
            } else {
                log!(DEBUG, "No available peer\n");
            }
        }
    }
}
