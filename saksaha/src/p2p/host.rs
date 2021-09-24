use super::{discovery::Disc, peer_op::PeerOp};
use crate::{common::SakResult, err_res};
use clap;
use logger::log;

pub struct Host {
    disc: Disc,
    peer_op: PeerOp,
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

        let host = Host { disc, peer_op };

        Ok(host)
    }
}

impl Host {
    pub async fn start(self) -> SakResult<bool> {
        log!(DEBUG, "Starting host...\n");

        let (disc, peer_op) =
            tokio::join!(self.disc.start(), self.peer_op.start());

        let _ = match disc {
            Ok(d) => d,
            Err(err) => {
                return err_res!("Error starting discovery, err: {}", err);
            },
        };

        let _ = match peer_op {
            Ok(l) => l,
            Err(err) => {
                return err_res!("Error starting listener, err: {}", err);
            }
        };

        return Ok(true);
    }
}
