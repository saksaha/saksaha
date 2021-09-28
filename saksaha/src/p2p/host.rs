use std::sync::{Arc,};
use super::{discovery::Disc, peer_op::PeerOp, peer_store::PeerStore};
use crate::{common::SakResult, err_res, sync::Sync};
use clap;
use logger::log;
use tokio::{sync::Mutex};

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
}

impl Host {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
    ) -> SakResult<Host> {
        let host = Host {
            rpc_port,
            disc_port,
            bootstrap_peers,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start host...\n");

        let peer_store = Arc::new(Mutex::new(PeerStore::new(10)));
        let peer_store_clone = peer_store.clone();

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
        );

        tokio::spawn(async move {
            match disc.start().await {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error spawning disc, err: {}", err);
                }
            }
        });

        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(
            peer_store_clone,
        );

        tokio::spawn(async move {
            match peer_op.start().await {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error spawning disc, err: {}", err);
                }
            }
        });

        Ok(true)
    }
}
