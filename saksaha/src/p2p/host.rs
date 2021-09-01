use std::sync::Arc;

use super::{discovery::Disc, peer_op::PeerOp};
use crate::{common::SakResult, err_res, sync::Sync};
use clap;
use logger::log;

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    // bootstrap_peers: Option<Vec<String>>,
}

impl Host {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        // bootstrap_peers: Option<Vec<String>>,
        // public_key: String,
        // secret: String,
    ) -> SakResult<Host> {
        // let rpc_port = match rpc_port {
        //     Some(p) => {
        //         if let Err(err) = p.parse::<usize>() {
        //             return err_res!(
        //                 "Error parsing the rpc port, err: {}",
        //                 err
        //             );
        //         }
        //         p.parse::<usize>().unwrap()
        //     }
        //     None => 0,
        // };

        // let disc_port = match disc_port {
        //     Some(p) => {
        //         if let Err(err) = p.parse::<usize>() {
        //             return err_res!(
        //                 "ERror parsing the rpc port, err: {}",
        //                 err
        //             );
        //         }
        //         p.parse::<usize>().unwrap()
        //     }
        //     None => 0,
        // };

        // let bootstrap_peers = match bootstrap_peers {
        //     Some(b) => b.map(str::to_string).collect(),
        //     None => Vec::new(),
        // };

        let host = Host { rpc_port, disc_port, /*bootstrap_peers */};

        // let disc = Disc::new(disc_port, bootstrap_peers);

        // let peer_op = match PeerOp::new() {
        //     Ok(p) => p,
        //     Err(err) => {
        //         return err_res!("Error initializing peer_op, err: {}", err);
        //     }
        // };

        // let host = Host { disc, peer_op };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start host...\n");
        // let host = Arc::new(self);
        // let cloned = host.clone();

        // tokio::spawn(async move {
        //     cloned.disc.start().await;
        // });

        // let cloned = host.clone();
        // tokio::spawn(async move {
        //     cloned.peer_op.start().await;
        // });

        Ok(true)
    }
}
