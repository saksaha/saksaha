mod dial;
mod listen;

use crate::{common::SakResult, err_res};
use logger::log;
use std::future::Future;
use tokio::{net::TcpListener, signal::ctrl_c, task::JoinHandle};

pub struct Disc {
    disc_port: usize,
}

impl Disc {
    pub fn new(disc_port: usize, bootstrap_peers: Vec<String>) -> Self {
        Disc { disc_port }
    }
}

impl Disc {
    pub async fn start(self) -> SakResult<bool> {
        tokio::spawn(async move {
            tokio::join!(
                self.start_dialing(),
                self.start_listening(),
            )
        });

        Ok(true)
    }
}
