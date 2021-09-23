use super::{discovery::Disc, listener::Listener};
use crate::{common::SakResult, err_res};
use clap;
use logger::log;

pub struct Host {
    disc: Disc,
    listener: Listener,
}

impl Host {
    pub fn new(
        rpc_port: Option<&str>,
        disc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> SakResult<Self> {
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

        let listener = match Listener::new() {
            Ok(l) => l,
            Err(err) => {
                return err_res!("Error initializing listener, err: {}", err);
            }
        };

        let host = Host { disc, listener };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Starting host...\n");

        let (disc, listener) =
            tokio::join!(self.disc.start(), self.listener.start());

        let _ = match disc {
            Ok(d) => d,
            Err(err) => {
                return err_res!("Error starting discovery, err: {}", err);
            },
        };

        let _ = match listener {
            Ok(l) => l,
            Err(err) => {
                return err_res!("Error starting listener, err: {}", err);
            }
        };

        return Ok(true);
    }
}
