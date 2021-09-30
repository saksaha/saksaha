use crate::{common::SakResult, err_res};
use logger::log;
use tokio::{net::TcpListener, sync::oneshot::Sender};

pub struct Listen;

impl Listen {
    pub async fn start_listening(&self, tx: Sender<u16>) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:0");

        log!(DEBUG, "Start p2p listening, addr: {}\n", local_addr);

        let listener = match TcpListener::bind(local_addr).await {
            Ok(l) => {
                match l.local_addr() {
                    Ok(addr) => match tx.send(addr.port()) {
                        Ok(_) => (),
                        Err(err) => {
                            return err_res!(
                                "Error sending peer op port, err: {}",
                                err
                            );
                        }
                    },
                    Err(err) => {
                        return err_res!(
                            "Error getting peer op local addr, err: {}",
                            err
                        );
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
