mod dial;
mod listen;
mod whoareyou;

use crate::{common::SakResult, err_res};
use logger::log;
use std::{sync::{Arc}};
use super::{peer_store::PeerStore};
use tokio::{sync::Mutex};

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
        Disc { disc_port, bootstrap_peers, peer_store }
    }

    pub async fn start(self) -> SakResult<bool> {
        let clone = self.peer_store.clone();
        let listen = listen::Listen {
            disc_port: self.disc_port,
            peer_store: clone,
        };

        tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!("Error start disc listening, err: {}", err);
                },
            }
        });

        let dialer = dial::Dial {
            bootstrap_peers: self.bootstrap_peers,
        };

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
