mod dial;
mod listen;

use crate::{common::SakResult, err_res};
use logger::log;
use std::{future::Future, sync::Arc};
use tokio::{net::TcpListener, signal::ctrl_c, task::JoinHandle};

pub struct Disc {
    pub disc_port: usize,
    pub bootstrap_peers: Option<Vec<String>>,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
    ) -> Self {
        Disc { disc_port, bootstrap_peers }
    }

    pub async fn start(self) -> SakResult<bool> {
        let listen = listen::Listen {
            disc_port: self.disc_port,
        };


        let listen_handle = tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!("Error start disc listening, err: {}", err);
                }
            }
        });

        let dialer = dial::Dial {
            bootstrap_peers: self.bootstrap_peers,
        };

        let dial_handle = tokio::spawn(async move {
            match dialer.start_dialing().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!("Error start disc dialing, err: {}", err);
                }
            }
        });

        if let Err(_) = listen_handle.await {
            // Shall we re-spawn?
        }

        if let Err(_) = dial_handle.await {
            // Shall we re-spawn?
        }

        Ok(true)
    }
}
