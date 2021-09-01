mod dial;
mod listen;

use crate::{common::SakResult, err_res};
use logger::log;
use std::{future::Future, sync::Arc};
use tokio::{net::TcpListener, signal::ctrl_c, task::JoinHandle};

#[derive(Clone, Copy)]
pub struct Disc {
    disc_port: usize,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_peers: Vec<String>
    ) -> Self {
        Disc { disc_port }
    }
}

impl Disc {
    pub async fn start(self) -> SakResult<bool> {
        let disc = Arc::new(self);
        let clone = disc.clone();
        tokio::spawn(async move {
            clone.start_listening().await;
        });

        let clone = disc.clone();
        tokio::spawn(async move {
            clone.start_dialing().await;
        });

        Ok(true)
    }
}
