use std::sync::Arc;

use crate::{common::SakResult, err_res};
use logger::log;
use tokio::{net::TcpListener, sync::mpsc::Sender};

pub struct Listen {
    peer_op_port_tx: Arc<Sender<u16>>,
    dial_loop_tx: Arc<Sender<usize>>,
}

impl Listen {
    pub fn new(
        peer_op_port_tx: Arc<Sender<u16>>,
        dial_loop_tx: Arc<Sender<usize>>,
    ) -> Listen {
        Listen {
            peer_op_port_tx,
            dial_loop_tx,
        }
    }

    pub async fn start_listening(&self) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:0");

        let listener = match TcpListener::bind(local_addr).await {
            Ok(l) => {
                let local_addr = match l.local_addr() {
                    Ok(addr) => addr,
                    Err(err) => {
                        return err_res!(
                            "Error getting peer op local addr, err: {}",
                            err
                        );
                    }
                };

                log!(DEBUG, "Start peer op listening, addr: {}\n", local_addr);

                match self.peer_op_port_tx.send(local_addr.port()).await {
                    Ok(_) => (),
                    Err(err) => {
                        // todo
                    }
                };

                l
            }
            Err(_) => {
                return err_res!("Error start peer_op listening");
            }
        };

        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return err_res!("Error accepting a request, err: {}", err);
                }
            };

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // let n = match
                }
            });
        }
    }
}
