mod dial;
mod listen;
mod whoareyou;

use super::peer_store::PeerStore;
use crate::{common::SakResult, err_res};
use logger::log;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Disc {
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    peer_store: Arc<Mutex<PeerStore>>,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        peer_store: Arc<Mutex<PeerStore>>,
    ) -> Self {
        Disc {
            disc_port,
            bootstrap_peers,
            peer_store,
        }
    }

    pub async fn start(self) -> SakResult<bool> {
        let peer_store = self.peer_store.clone();
        let listen = listen::Listen::new(self.disc_port, peer_store);

        let listen_handle = tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(1),
                Err(err) => {
                    return err_res!("Error start disc listening, err: {}", err);
                }
            }
        });

        match listen_handle.await {
            Ok(handle) => {
                match handle {
                    Ok(_) => (),
                    Err(err) => {
                        return err_res!("Disc listen thread has returned \
                            with error, err: {}", err);
                    }
                }
            }
            Err(err) => {
                return err_res!("Error spawning disc listen, err: {}", err);
            }
        }

        let peer_store = self.peer_store.clone();
        let dialer = dial::Dial::new(self.bootstrap_peers, peer_store);

        tokio::spawn(async move {
            match dialer.start_dialing().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!("Error start disc dialing, err: {}", err);
                }
            }
        });

        Ok(true)
    }
}
