use std::sync::{Arc,};
use super::{discovery::Disc, peer_op::PeerOp, peer_store::PeerStore};
use crate::{common::SakResult, err_res, sync::Sync};
use clap;
use logger::log;
use tokio::{sync::{Mutex, mpsc::Sender}, task::JoinHandle};

// type HostReturn =

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
    pub async fn start(&self, tx: Sender<bool>) -> SakResult<()> {
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
                Ok(_) => Ok(true),
                Err(err) => {
                    println!("44444444444");
                    tx.send(false).await;
                    Err(err)
                }
            }
        });

        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(
            peer_store_clone,
        );

        tokio::spawn(async move {
            match peer_op.start().await {
                Ok(_) => Ok(true),
                Err(err) => {
                    Err(err)
                }
            }
        });

        Ok(())
    }
}
