use std::sync::Arc;

use super::{discovery::Disc, peer_op::PeerOp};
use crate::{common::SakResult, err_res, sync::Sync};
use clap;
use logger::log;

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

        let disc = Disc {
            disc_port: self.disc_port,
            bootstrap_peers: self.bootstrap_peers.to_owned(),
        };

        tokio::spawn(async move {
            match disc.start().await {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error spawning disc, err: {}", err);
                }
            }
        });

        let peer_op = PeerOp {

        };

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
