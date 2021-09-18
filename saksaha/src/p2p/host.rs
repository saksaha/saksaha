use super::discovery::Disc;
use crate::{common::SakResult, err_res, sync::ThreadPool};
use clap;
use logger::log;

pub struct Host {
    disc: Disc,
}

impl Host {
    pub fn new(
        rpc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> SakResult<Self> {
        let rpc_port = match rpc_port {
            Some(p) => {
                if let Err(err) = p.parse::<usize>() {
                    return err_res!("Error parsing the rpc port, err: {}", err);
                }
                p.parse::<usize>().unwrap()
            }
            None => 0,
        };

        let bootstrap_peers = match bootstrap_peers {
            Some(b) => b.map(str::to_string).collect(),
            None => Vec::new(),
        };

        let tpool = ThreadPool::new(2)?;

        let disc = Disc::new(tpool, bootstrap_peers);

        Ok(Host { disc })
    }
}

impl Host {
    pub fn start(&self) {
        log!(DEBUG, "Starting host...\n");

        self.disc.start();
    }
}
