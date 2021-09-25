use super::{discovery::Disc, peer_op::PeerOp};
use crate::{common::SakResult, err_res, sync::Sync};
use clap;
use logger::log;

pub struct Host {
    disc: Disc,
    // peer_op: PeerOp,
}

impl Host {
    pub fn new(
        rpc_port: Option<&str>,
        disc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> SakResult<Host> {
        let rpc_port = match rpc_port {
            Some(p) => {
                if let Err(err) = p.parse::<usize>() {
                    return err_res!(
                        "Error parsing the rpc port, err: {}",
                        err
                    );
                }
                p.parse::<usize>().unwrap()
            }
            None => 0,
        };

        let disc_port = match disc_port {
            Some(p) => {
                if let Err(err) = p.parse::<usize>() {
                    return err_res!(
                        "ERror parsing the rpc port, err: {}",
                        err
                    );
                }
                p.parse::<usize>().unwrap()
            }
            None => 0,
        };

        let bootstrap_peers = match bootstrap_peers {
            Some(b) => b.map(str::to_string).collect(),
            None => Vec::new(),
        };

        let disc = Disc::new(disc_port, bootstrap_peers);

        let peer_op = match PeerOp::new() {
            Ok(p) => p,
            Err(err) => {
                return err_res!("Error initializing peer_op, err: {}", err);
            }
        };

        let host = Host { disc,  };

        Ok(host)
    }
}

impl Host {
    pub async fn start(self) -> SakResult<bool> {
        log!(DEBUG, "Start host...\n");

        // let a = std::sync::Arc::new(self.disc);
        // let a = a.clone();

        tokio::spawn(async move {
            self.disc.start().await;
        });

        // let disc = std::sync::Arc::new(&self.disc);
        // let disc = disc.clone();

        // let disc = std::sync::Arc::new(Disc {});
        // let disc = disc.clone();

        // let a = std::sync::Arc::new(self);
        // let b = a.clone();

        // tokio::spawn(async move {
        //     disc.start().await;
        // });

        // tokio::join!(self.disc.start(), self.peer_op.start());


        Ok(true)
    }
}
